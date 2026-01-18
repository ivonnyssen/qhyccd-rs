
use eyre::{eyre, Result};

use crate::backend::{read_lock, CameraBackend, QHYCCDHandle};
use crate::QHYError::*;

#[cfg(feature = "simulation")]
use crate::CCDChipArea;

#[cfg(not(test))]
use libqhyccd_sys::{CloseQHYCCD, InitQHYCCD, OpenQHYCCD, QHYCCD_SUCCESS};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{CloseQHYCCD, InitQHYCCD, OpenQHYCCD, QHYCCD_SUCCESS};

use super::Camera;

impl Camera {
    /// Opens a camera with the given id. The SDK automatically finds all connected cameras upon initialization
    /// but does not call open on the cameras. You have to call open on the camera you want to use. Calling open
    /// on a camera that is already open does not do anything.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// ```
    pub fn open(&self) -> Result<()> {
        if self.is_open()? {
            return Ok(());
        }
        match &self.backend {
            CameraBackend::Real { handle } => {
                // read and see if the handle is already Some(_)
                let mut lock = handle.write().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire write lock on camera handle")
                })?;
                unsafe {
                    match std::ffi::CString::new(self.id.clone()) {
                        Ok(c_id) => {
                            let handle = OpenQHYCCD(c_id.as_ptr());
                            if handle.is_null() {
                                let error = OpenCameraError;
                                tracing::error!(error = ?error);
                                return Err(eyre!(error));
                            }
                            *lock = Some(QHYCCDHandle { ptr: handle });
                            Ok(())
                        }
                        Err(error) => {
                            tracing::error!(error = ?error);
                            Err(eyre!(error))
                        }
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let mut state = state.write().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire write lock on simulated camera state")
                })?;
                state.is_open = true;
                Ok(())
            }
        }
    }

    /// Closes the camera. If you have to call this function, you can then open the camera again by
    /// calling `open`. Calling close on a camera that is not open does not do anything.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.close().expect("close failed");
    /// ```
    pub fn close(&self) -> Result<()> {
        if !self.is_open()? {
            return Ok(());
        }
        match &self.backend {
            CameraBackend::Real { handle } => {
                let mut lock = handle.write().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire write lock on camera handle")
                })?;

                match *lock {
                    Some(handle) => match unsafe { CloseQHYCCD(handle.ptr) } {
                        QHYCCD_SUCCESS => {
                            lock.take();
                            Ok(())
                        }
                        error_code => {
                            let error = CloseCameraError { error_code };
                            tracing::error!(error = ?error);
                            Err(eyre!(error))
                        }
                    },
                    None => Ok(()),
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let mut state = state.write().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire write lock on simulated camera state")
                })?;
                state.is_open = false;
                state.is_initialized = false;
                Ok(())
            }
        }
    }

    /// initializes the camera to a new session - use this to change from LiveMode to SingleFrameMode for instance
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// ```
    pub fn init(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, InitCameraError { error_code: 0 })?;

                match unsafe { InitQHYCCD(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = InitCameraError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let mut state = state.write().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire write lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                state.is_initialized = true;
                // Reset ROI to full frame based on current readout mode
                let (width, height) = state
                    .config
                    .readout_modes
                    .get(state.readout_mode as usize)
                    .map(|(_, res)| *res)
                    .unwrap_or((
                        state.config.chip_info.image_width,
                        state.config.chip_info.image_height,
                    ));
                state.roi = CCDChipArea {
                    start_x: 0,
                    start_y: 0,
                    width,
                    height,
                };
                Ok(())
            }
        }
    }

    /// Returns `true` if the camera is open
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found"); // this does not open the camera
    /// camera.open().expect("open failed");
    /// let is_open = camera.is_open();
    /// println!("Is camera open: {:?}", is_open);
    /// ```
    pub fn is_open(&self) -> Result<bool> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let lock = handle.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on camera handle")
                })?;
                Ok((*lock).is_some())
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                Ok(state.is_open)
            }
        }
    }
}
