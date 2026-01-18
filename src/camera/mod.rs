mod configuration;
mod imaging;
mod info;
mod lifecycle;
mod parameters;
mod readout_modes;

use std::sync::{Arc, RwLock};

use crate::backend::CameraBackend;

#[cfg(feature = "simulation")]
use crate::simulation::{self, SimulatedCameraState};

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq)]
/// The representation of a camera. It is constructed by the SDK and can be used to
/// interact with the camera.
pub struct Camera {
    id: String,
    #[educe(PartialEq(ignore))]
    backend: CameraBackend,
}

impl Camera {
    /// Creates a new instance of the camera. The Sdk automatically finds all cameras and provides them in it's cameras() iterator. Creating
    /// a camera manually should only be needed for rare cases.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, Camera};
    /// let camera = Camera::new("camera id from sdk".to_string());
    /// println!("Camera: {:?}", camera);
    /// ```
    pub fn new(id: String) -> Self {
        Self {
            id,
            backend: CameraBackend::Real {
                handle: Arc::new(RwLock::new(None)),
            },
        }
    }

    /// Creates a new simulated camera instance
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Camera;
    /// use qhyccd_rs::simulation::SimulatedCameraConfig;
    ///
    /// let config = SimulatedCameraConfig::default()
    ///     .with_filter_wheel(5)
    ///     .with_cooler();
    /// let camera = Camera::new_simulated(config);
    /// ```
    #[cfg(feature = "simulation")]
    pub fn new_simulated(config: simulation::SimulatedCameraConfig) -> Self {
        let id = config.id.clone();
        Self {
            id,
            backend: CameraBackend::Simulated {
                state: Arc::new(RwLock::new(SimulatedCameraState::new(config))),
            },
        }
    }

    /// Returns true if this is a simulated camera
    #[cfg(feature = "simulation")]
    pub fn is_simulated(&self) -> bool {
        matches!(self.backend, CameraBackend::Simulated { .. })
    }

    /// Returns true if this is a simulated camera (always false without simulation feature)
    #[cfg(not(feature = "simulation"))]
    pub fn is_simulated(&self) -> bool {
        false
    }

    /// Returns the id of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// println!("Camera id: {}", camera.id());
    /// ```
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}
