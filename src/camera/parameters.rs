#![allow(unused_unsafe)]

use eyre::{eyre, Result};

use crate::backend::{read_lock, CameraBackend};
use crate::{Control, QHYError::*};

#[cfg(not(test))]
use libqhyccd_sys::{
    GetQHYCCDParam, GetQHYCCDParamMinMaxStep, IsQHYCCDCFWPlugged, IsQHYCCDControlAvailable,
    SetQHYCCDParam, QHYCCD_ERROR, QHYCCD_ERROR_F64, QHYCCD_SUCCESS,
};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDParam, GetQHYCCDParamMinMaxStep, IsQHYCCDCFWPlugged, IsQHYCCDControlAvailable,
    SetQHYCCDParam, QHYCCD_ERROR, QHYCCD_ERROR_F64, QHYCCD_SUCCESS,
};

use super::Camera;

impl Camera {
    /// Returns information about the control given to the function
    /// # Returns
    /// `Err` if the control is not available
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let control = camera.is_control_available(Control::Exposure).expect("is_control_available failed");
    /// println!("Control: {:?}", control);
    /// ```
    pub fn is_control_available(&self, control: Control) -> Option<u32> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = match read_lock!(handle, IsControlAvailableError { control }) {
                    Ok(handle) => handle,
                    Err(_) => return None,
                };
                match unsafe { IsQHYCCDControlAvailable(handle, control as u32) } {
                    QHYCCD_ERROR => {
                        let error = IsControlAvailableError { control };
                        tracing::debug!(control = ?error);
                        None
                    }
                    is_supported => Some(is_supported),
                }
            }
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => {
                let state = match state.read() {
                    Ok(state) => state,
                    Err(_) => return None,
                };
                if !state.is_open {
                    return None;
                }
                // Check if control is in supported_controls
                if state.config.supported_controls.contains_key(&control) {
                    // For CamColor, return the bayer mode value
                    if control == Control::CamColor {
                        return state.config.bayer_mode.map(|m| m as u32);
                    }
                    Some(1) // Control is available
                } else {
                    None
                }
            }
        }
    }

    /// Returns the value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let exposure = camera.get_parameter(Control::Exposure).expect("get_parameter failed");
    /// println!("Exposure: {}", exposure);
    /// ```
    pub fn get_parameter(&self, control: Control) -> Result<f64> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetParameterError { control })?;
                let res = unsafe { GetQHYCCDParam(handle, control as u32) };
                if (res - QHYCCD_ERROR_F64).abs() < f64::EPSILON {
                    let error = GetParameterError { control };
                    tracing::error!(error = ?error);
                    Err(eyre!(error))
                } else {
                    Ok(res)
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
                // Handle special controls
                match control {
                    Control::CfwPort => {
                        // Return position as ASCII value (48 = '0')
                        Ok((state.filter_wheel_position + 48) as f64)
                    }
                    Control::CfwSlotsNum => Ok(state.config.filter_wheel_slots as f64),
                    Control::CurTemp => Ok(state.current_temperature),
                    Control::CurPWM => Ok(state.cooler_pwm),
                    Control::Cooler => {
                        if state
                            .config
                            .supported_controls
                            .contains_key(&Control::Cooler)
                        {
                            Ok(state.target_temperature)
                        } else {
                            let error = GetParameterError { control };
                            tracing::error!(error = ?error);
                            Err(eyre!(error))
                        }
                    }
                    _ => state.parameters.get(&control).copied().ok_or_else(|| {
                        let error = GetParameterError { control };
                        tracing::error!(error = ?error);
                        eyre!(error)
                    }),
                }
            }
        }
    }

    /// Returns the min, max and step value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let (min_exposure, max_exposure, exposure_resolution) = camera.get_parameter_min_max_step(Control::Exposure).expect("getting min,max,step failed");
    /// ```
    pub fn get_parameter_min_max_step(&self, control: Control) -> Result<(f64, f64, f64)> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetMinMaxStepError { control })?;
                let mut min: f64 = 0.0;
                let mut max: f64 = 0.0;
                let mut step: f64 = 0.0;
                match unsafe {
                    GetQHYCCDParamMinMaxStep(
                        handle,
                        control as u32,
                        &mut min as *mut f64,
                        &mut max as *mut f64,
                        &mut step as *mut f64,
                    )
                } {
                    QHYCCD_SUCCESS => Ok((min, max, step)),
                    _ => {
                        let error = GetMinMaxStepError { control };
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
                state
                    .config
                    .supported_controls
                    .get(&control)
                    .copied()
                    .ok_or_else(|| {
                        let error = GetMinMaxStepError { control };
                        tracing::error!(error = ?error);
                        eyre!(error)
                    })
            }
        }
    }

    /// Sets the value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_parameter failed");
    /// ```
    pub fn set_parameter(&self, control: Control, value: f64) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, SetParameterError { error_code: 0 })?;
                match unsafe { SetQHYCCDParam(handle, control as u32, value) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = SetParameterError { error_code };
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
                // Handle special controls
                match control {
                    Control::CfwPort => {
                        // Value is ASCII position, convert to 0-indexed
                        state.filter_wheel_position = (value as u32).saturating_sub(48);
                    }
                    Control::Cooler => {
                        state.target_temperature = value;
                    }
                    Control::ManualPWM => {
                        state.cooler_pwm = value;
                    }
                    Control::Exposure => {
                        state.exposure_duration_us = value as u64;
                        state.parameters.insert(control, value);
                    }
                    _ => {
                        state.parameters.insert(control, value);
                    }
                }
                Ok(())
            }
        }
    }

    /// Convinience function that sets the value for a given control if it is available
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_if_available(Control::TransferBit, 16.0).expect("failed to set usb transfer mode");
    /// ```
    pub fn set_if_available(&self, control: Control, value: f64) -> Result<()> {
        match self.is_control_available(control) {
            Some(_) => self.set_parameter(control, value),
            None => Err(eyre!(IsControlAvailableError { control })),
        }
    }

    /// Returns `true` if a filter wheel is plugged into the given camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let is_cfw_plugged_in = camera.is_cfw_plugged_in().expect("is_cfw_plugged_in failed");
    /// println!("Is filter wheel plugged in: {}", is_cfw_plugged_in);
    /// ```
    pub fn is_cfw_plugged_in(&self) -> Result<bool> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, IsCfwPluggedInError)?;
                match unsafe { IsQHYCCDCFWPlugged(handle) } {
                    QHYCCD_SUCCESS => Ok(true),
                    QHYCCD_ERROR => Ok(false),
                    _ => {
                        let error = IsCfwPluggedInError;
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
                Ok(state.config.filter_wheel_slots > 0)
            }
        }
    }
}
