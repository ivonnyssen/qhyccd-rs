use std::ffi::{c_char, CStr};

use eyre::{eyre, Result};

use crate::backend::{read_lock, CameraBackend};
use crate::{CCDChipArea, CCDChipInfo, QHYError::*};

#[cfg(not(test))]
use libqhyccd_sys::{
    GetQHYCCDChipInfo, GetQHYCCDEffectiveArea, GetQHYCCDFWVersion, GetQHYCCDModel,
    GetQHYCCDOverScanArea, GetQHYCCDType, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDChipInfo, GetQHYCCDEffectiveArea, GetQHYCCDFWVersion, GetQHYCCDModel,
    GetQHYCCDOverScanArea, GetQHYCCDType, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

use super::Camera;

impl Camera {
    /// Returns the model of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let model = camera.get_model().expect("get_model failed");
    /// println!("Camera model: {}", model);
    /// ```
    pub fn get_model(&self) -> Result<String> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetCameraModelError { error_code: 0 })?;
                let mut model: [c_char; 80] = [0; 80];
                match unsafe { GetQHYCCDModel(handle, model.as_mut_ptr()) } {
                    QHYCCD_SUCCESS => {
                        let model = match unsafe { CStr::from_ptr(model.as_ptr()) }.to_str() {
                            Ok(model) => model,
                            Err(error) => {
                                tracing::error!(error = ?error);
                                return Err(eyre!(error));
                            }
                        };
                        Ok(model.to_string())
                    }
                    error_code => {
                        let error = GetCameraModelError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.model.clone())
            }
        }
    }

    /// Returns the firmware version of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let firmware_version = camera.get_firmware_version().expect("get_firmware_version failed");
    /// println!("Firmware version: {}", firmware_version);
    /// ```
    pub fn get_firmware_version(&self) -> Result<String> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetFirmwareVersionError { error_code: 0 })?;
                let mut version = [0u8; 32];
                match unsafe { GetQHYCCDFWVersion(handle, version.as_mut_ptr()) } {
                    QHYCCD_SUCCESS => {
                        if version[0] >> 4 <= 9 {
                            Ok(format!(
                                "Firmware version: 20{}_{}_{}",
                                (((version[0] >> 4) + 0x10) as u32),
                                version[0] & 0x0F,
                                version[1]
                            ))
                        } else {
                            Ok(format!(
                                "Firmware version: 20{}_{}_{}",
                                ((version[0] >> 4) as u32),
                                version[0] & 0x0F,
                                version[1]
                            ))
                        }
                    }
                    error_code => {
                        let error = GetFirmwareVersionError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.firmware_version.clone())
            }
        }
    }

    /// Not sure what this does, for a QHY178M Cool it returns 4010
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let camera_type = camera.get_type().expect("get_type failed");
    /// println!("Camera type: {}", camera_type);
    /// ```
    pub fn get_type(&self) -> Result<u32> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetCameraTypeError)?;
                match unsafe { GetQHYCCDType(handle) } {
                    QHYCCD_ERROR => {
                        let error = GetCameraTypeError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    camera_type => Ok(camera_type),
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.camera_type)
            }
        }
    }

    /// Returns the CCD chip information
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let ccd_info = camera.get_ccd_info().expect("get_ccd_info failed");
    /// println!("CCD info: {:?}", ccd_info);
    /// ```
    pub fn get_ccd_info(&self) -> Result<CCDChipInfo> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetCCDInfoError { error_code: 0 })?;
                let mut chipw: f64 = 0.0;
                let mut chiph: f64 = 0.0;
                let mut imagew: u32 = 0;
                let mut imageh: u32 = 0;
                let mut pixelw: f64 = 0.0;
                let mut pixelh: f64 = 0.0;
                let mut bpp: u32 = 0;
                match unsafe {
                    GetQHYCCDChipInfo(
                        handle,
                        &mut chipw as *mut f64,
                        &mut chiph as *mut f64,
                        &mut imagew as *mut u32,
                        &mut imageh as *mut u32,
                        &mut pixelw as *mut f64,
                        &mut pixelh as *mut f64,
                        &mut bpp as *mut u32,
                    )
                } {
                    QHYCCD_SUCCESS => Ok(CCDChipInfo {
                        chip_width: chipw,
                        chip_height: chiph,
                        image_width: imagew,
                        image_height: imageh,
                        pixel_width: pixelw,
                        pixel_height: pixelh,
                        bits_per_pixel: bpp,
                    }),
                    error_code => {
                        let error = GetCCDInfoError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.chip_info)
            }
        }
    }

    /// Get the overscan area of the chip
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let overscan_area = camera.get_overscan_area().expect("get_overscan_area failed");
    /// println!("Overscan area: {:?}", overscan_area);
    /// ```
    pub fn get_overscan_area(&self) -> Result<CCDChipArea> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetOverscanAreaError { error_code: 0 })?;
                let mut start_x: u32 = 0;
                let mut start_y: u32 = 0;
                let mut width: u32 = 0;
                let mut height: u32 = 0;
                match unsafe {
                    GetQHYCCDOverScanArea(
                        handle,
                        &mut start_x as *mut u32,
                        &mut start_y as *mut u32,
                        &mut width as *mut u32,
                        &mut height as *mut u32,
                    )
                } {
                    QHYCCD_SUCCESS => Ok(CCDChipArea {
                        start_x,
                        start_y,
                        width,
                        height,
                    }),
                    error_code => {
                        let error = GetOverscanAreaError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.overscan_area)
            }
        }
    }

    /// Get the effective imaging chip area
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let effective_area = camera.get_effective_area().expect("get_effective_area failed");
    /// println!("Effective area: {:?}", effective_area);
    /// ```
    pub fn get_effective_area(&self) -> Result<CCDChipArea> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetEffectiveAreaError { error_code: 0 })?;
                let mut start_x: u32 = 0;
                let mut start_y: u32 = 0;
                let mut width: u32 = 0;
                let mut height: u32 = 0;
                match unsafe {
                    GetQHYCCDEffectiveArea(
                        handle,
                        &mut start_x as *mut u32,
                        &mut start_y as *mut u32,
                        &mut width as *mut u32,
                        &mut height as *mut u32,
                    )
                } {
                    QHYCCD_SUCCESS => Ok(CCDChipArea {
                        start_x,
                        start_y,
                        width,
                        height,
                    }),
                    error_code => {
                        let error = GetEffectiveAreaError { error_code };
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = state.read().map_err(|err| {
                    tracing::error!(error=?err);
                    eyre!("Could not acquire read lock on simulated camera state")
                })?;
                if !state.is_open {
                    return Err(eyre!(CameraNotOpenError));
                }
                Ok(state.config.effective_area)
            }
        }
    }
}
