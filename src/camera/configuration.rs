use eyre::{eyre, Result};

use crate::backend::{read_lock, CameraBackend};
use crate::{CCDChipArea, StreamMode, QHYError::*};

#[cfg(feature = "simulation")]
use crate::simulation;

#[cfg(not(test))]
use libqhyccd_sys::{
    SetQHYCCDBinMode, SetQHYCCDBitsMode, SetQHYCCDDebayerOnOff, SetQHYCCDReadMode,
    SetQHYCCDResolution, SetQHYCCDStreamMode, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    SetQHYCCDBinMode, SetQHYCCDBitsMode, SetQHYCCDDebayerOnOff, SetQHYCCDReadMode,
    SetQHYCCDResolution, SetQHYCCDStreamMode, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

use super::Camera;

impl Camera {
    /// Sets the stream mode of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// ```
    pub fn set_stream_mode(&self, mode: StreamMode) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetStreamModeError { error_code: 0 })?;
                match unsafe { SetQHYCCDStreamMode(handle, mode as u8) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetStreamModeError { error_code };
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
                state.stream_mode = Some(mode);
                Ok(())
            }
        }
    }

    /// Sets the readout mode of the camera with the id of the `ReadoutMode` between 0 and the value
    /// returned by `get_number_of_readout_modes`
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_readout_mode(0).expect("set_readout_mode failed");
    /// ```
    pub fn set_readout_mode(&self, mode: u32) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetReadoutModeError { error_code: 0 })?;
                match unsafe { SetQHYCCDReadMode(handle, mode) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetReadoutModeError { error_code };
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
                if mode as usize >= state.config.readout_modes.len() {
                    return Err(eyre!(SetReadoutModeError {
                        error_code: QHYCCD_ERROR
                    }));
                }
                state.readout_mode = mode;
                Ok(())
            }
        }
    }

    /// Sets the binning mode of the camera
    /// Only symmetric binnings are supported
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_bin_mode(1, 1).expect("set_bin_mode failed");
    /// ```
    pub fn set_bin_mode(&self, bin_x: u32, bin_y: u32) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetBinModeError { error_code: 0 })?;
                match unsafe { SetQHYCCDBinMode(handle, bin_x, bin_y) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetBinModeError { error_code };
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
                state.binning = (bin_x, bin_y);
                Ok(())
            }
        }
    }

    /// According to c-cod ethis does not work for all cameras
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_debayer(true).expect("set_debayer failed");
    /// ```
    pub fn set_debayer(&self, on: bool) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetDebayerError { error_code: 0 })?;
                match unsafe { SetQHYCCDDebayerOnOff(handle, on) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetDebayerError { error_code };
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
                state.debayer_enabled = on;
                Ok(())
            }
        }
    }

    /// Sets the Region of interest of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera, CCDChipArea};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let roi = CCDChipArea {
    ///     start_x: 0,
    ///     start_y: 0,
    ///     width: 640,
    ///     height: 480,
    /// };
    /// camera.set_roi(roi).expect("set_roi failed");
    /// ```
    pub fn set_roi(&self, roi: CCDChipArea) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetRoiError { error_code: 0 })?;
                match unsafe {
                    SetQHYCCDResolution(handle, roi.start_x, roi.start_y, roi.width, roi.height)
                } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetRoiError { error_code };
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
                state.roi = roi;
                Ok(())
            }
        }
    }

    /// Sets the USB transfer mode to either 8 or 16 bit
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_bit_mode(16).expect("set_bit_mode failed");
    /// ```
    pub fn set_bit_mode(&self, mode: u32) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetBitModeError { error_code: 0 })?;
                match unsafe { SetQHYCCDBitsMode(handle, mode) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetBitModeError { error_code };
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
                state.bit_depth = mode;
                Ok(())
            }
        }
    }
}
