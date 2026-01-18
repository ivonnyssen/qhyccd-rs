use std::ffi::{c_char, CStr};

use eyre::{eyre, Result};
use tracing::error;

use crate::{Camera, FilterWheel, QHYError::*, SDKVersion};

#[cfg(feature = "simulation")]
use crate::simulation;

// These imports are only used when NOT simulating (real hardware path in Sdk::new)
#[cfg(all(not(test), not(feature = "simulation")))]
use libqhyccd_sys::{GetQHYCCDId, InitQHYCCDResource, ScanQHYCCD};

#[cfg(not(test))]
use libqhyccd_sys::{GetQHYCCDSDKVersion, ReleaseQHYCCDResource, QHYCCD_ERROR, QHYCCD_SUCCESS};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDId, GetQHYCCDSDKVersion, InitQHYCCDResource, ReleaseQHYCCDResource, ScanQHYCCD,
    QHYCCD_ERROR, QHYCCD_SUCCESS,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
/// The representation of the SDK. It automatically allocates the SDK when constructed
/// and automatically frees resource when deconstructed.
///
/// # Example
/// ```no_run
/// use qhyccd_rs::Sdk;
///
/// let sdk = Sdk::new().expect("SDK::new failed");
/// let sdk_version = sdk.version().expect("get_sdk_version failed");
/// println!("SDK version: {:?}", sdk_version);
/// let camera = sdk.cameras().last().expect("no camera found");
/// println!("Camera: {:?}", camera);
/// let sdk = Sdk::new().expect("SDK::new failed");
/// println!("{} filter wheels connected.", sdk.filter_wheels().count());
/// ```
pub struct Sdk {
    cameras: Vec<Camera>,
    filter_wheels: Vec<FilterWheel>,
    #[cfg(feature = "simulation")]
    is_simulated: bool,
}

#[allow(unused_unsafe)]
impl Sdk {
    /// Creates a new instance of the SDK
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new();
    /// assert!(sdk.is_ok());
    /// ```
    #[cfg(not(feature = "simulation"))]
    pub fn new() -> Result<Self> {
        match unsafe { InitQHYCCDResource() } {
            QHYCCD_SUCCESS => {
                let num_cameras = match unsafe { ScanQHYCCD() } {
                    QHYCCD_ERROR => {
                        let error = ScanQHYCCDError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    num => Ok(num),
                }?;

                let mut cameras = Vec::with_capacity(num_cameras as usize);
                let mut filter_wheels = Vec::with_capacity(num_cameras as usize);
                for index in 0..num_cameras {
                    let id = {
                        let mut c_id: [c_char; 32] = [0; 32];
                        unsafe {
                            match GetQHYCCDId(index, c_id.as_mut_ptr()) {
                                QHYCCD_SUCCESS => {
                                    let id = match CStr::from_ptr(c_id.as_ptr()).to_str() {
                                        Ok(id) => id,
                                        Err(error) => {
                                            tracing::error!(error = ?error);
                                            return Err(eyre!(error));
                                        }
                                    };
                                    Ok(id.to_owned())
                                }
                                error_code => {
                                    let error = GetCameraIdError { error_code };
                                    tracing::error!(error = ?error);
                                    Err(eyre!(error))
                                }
                            }
                        }
                    }?;
                    let camera = Camera::new(id.clone());
                    let mut has_filter_wheel = false;
                    match camera.open() {
                        Ok(_) => match camera.is_cfw_plugged_in() {
                            Ok(true) => {
                                tracing::trace!("Camera {} reporting a filter wheel", id);
                                has_filter_wheel = true;
                            }
                            Ok(false) => {
                                tracing::trace!("Camera {} has no filter wheel", id)
                            }
                            Err(error) => {
                                tracing::error!(error = ?error);
                            }
                        },
                        Err(error) => {
                            tracing::error!(error = ?error);
                            continue;
                        }
                    }
                    match camera.close() {
                        Ok(_) => (),
                        Err(error) => {
                            tracing::error!(error = ?error);
                            continue;
                        }
                    }
                    if has_filter_wheel {
                        filter_wheels.push(FilterWheel::new(Camera::new(id)))
                    };
                    cameras.push(camera);
                }

                Ok(Sdk {
                    cameras,
                    filter_wheels,
                    #[cfg(feature = "simulation")]
                    is_simulated: false,
                })
            }
            error_code => {
                let error = InitSDKError { error_code };
                tracing::error!(error = ?error);
                Err(eyre!(error))
            }
        }
    }

    /// Creates a new SDK instance with automatic simulation when the feature is enabled
    ///
    /// When compiled with the `simulation` feature, this automatically returns a simulated
    /// SDK with a default camera (QHY178M-Simulated) that includes a 7-position filter wheel
    /// and cooler support. This allows the same code to work seamlessly in both real and
    /// simulated environments.
    ///
    /// For custom simulated camera configurations, use `new_simulated()` and
    /// `add_simulated_camera()` instead.
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    ///
    /// // Same code works with or without simulation feature
    /// let sdk = Sdk::new().expect("Failed to initialize SDK");
    /// let cameras = sdk.cameras();
    /// println!("Found {} camera(s)", cameras.count());
    /// ```
    #[cfg(feature = "simulation")]
    pub fn new() -> Result<Self> {
        let mut sdk = Self::new_simulated();

        // Add default simulated camera with 7-position filter wheel and cooler
        let config = simulation::SimulatedCameraConfig::default()
            .with_id("SIM-QHY178M")
            .with_model("QHY178M-Simulated")
            .with_filter_wheel(7)
            .with_cooler();

        sdk.add_simulated_camera(config);

        Ok(sdk)
    }

    /// Creates a new SDK instance for simulation without scanning for real hardware
    ///
    /// This creates an empty SDK that can be populated with simulated cameras
    /// using `add_simulated_camera()`.
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::simulation::SimulatedCameraConfig;
    ///
    /// let mut sdk = Sdk::new_simulated();
    /// let config = SimulatedCameraConfig::default().with_filter_wheel(5);
    /// sdk.add_simulated_camera(config);
    ///
    /// assert_eq!(sdk.cameras().count(), 1);
    /// ```
    #[cfg(feature = "simulation")]
    pub fn new_simulated() -> Self {
        Self {
            cameras: Vec::new(),
            filter_wheels: Vec::new(),
            is_simulated: true,
        }
    }

    /// Adds a simulated camera to the SDK
    ///
    /// If the camera configuration includes a filter wheel (filter_wheel_slots > 0),
    /// a corresponding FilterWheel will also be added.
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::simulation::SimulatedCameraConfig;
    ///
    /// let mut sdk = Sdk::new_simulated();
    /// let config = SimulatedCameraConfig::default()
    ///     .with_id("SIM-001")
    ///     .with_filter_wheel(5);
    /// sdk.add_simulated_camera(config);
    ///
    /// let camera = sdk.cameras().next().unwrap();
    /// assert_eq!(camera.id(), "SIM-001");
    /// ```
    #[cfg(feature = "simulation")]
    pub fn add_simulated_camera(&mut self, config: simulation::SimulatedCameraConfig) {
        let has_filter_wheel = config.filter_wheel_slots > 0;
        let filter_wheel_slots = config.filter_wheel_slots;
        let id = config.id.clone();
        let camera = Camera::new_simulated(config);

        if has_filter_wheel {
            // Create a separate simulated camera instance for the filter wheel
            // This matches the pattern used for real hardware
            let fw_config = simulation::SimulatedCameraConfig::default()
                .with_id(&id)
                .with_filter_wheel(filter_wheel_slots);
            self.filter_wheels
                .push(FilterWheel::new(Camera::new_simulated(fw_config)));
        }

        self.cameras.push(camera);
    }

    /// Returns an iterator over all cameras found by the SDK
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// for camera in sdk.cameras() {
    ///    println!("Camera: {:?}", camera);
    /// }
    /// ```
    pub fn cameras(&self) -> impl Iterator<Item = &Camera> {
        self.cameras.iter()
    }

    /// Returns an iterator over all filter wheels found by the SDK
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// println!("{} filter wheels connected.", sdk.filter_wheels().count());
    /// ```
    pub fn filter_wheels(&self) -> impl Iterator<Item = &FilterWheel> {
        self.filter_wheels.iter()
    }

    /// Returns the version of the SDK
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let sdk_version = sdk.version().expect("get_sdk_version failed");
    /// println!("SDK version: {:?}", sdk_version);
    /// ```
    pub fn version(&self) -> Result<SDKVersion> {
        let mut year: u32 = 0;
        let mut month: u32 = 0;
        let mut day: u32 = 0;
        let mut subday: u32 = 0;
        match unsafe { GetQHYCCDSDKVersion(&mut year, &mut month, &mut day, &mut subday) } {
            QHYCCD_SUCCESS => Ok(SDKVersion {
                year,
                month,
                day,
                subday,
            }),
            error_code => {
                let error = GetSDKVersionError { error_code };
                tracing::error!(error = ?error);
                Err(eyre!(error))
            }
        }
    }
}

#[allow(unused_unsafe)]
impl Drop for Sdk {
    fn drop(&mut self) {
        // Skip FFI cleanup for simulated SDKs
        #[cfg(feature = "simulation")]
        if self.is_simulated {
            return;
        }

        match unsafe { ReleaseQHYCCDResource() } {
            QHYCCD_SUCCESS => (),
            error_code => {
                let error = CloseSDKError { error_code };
                tracing::error!(error = ?error);
            }
        }
    }
}
