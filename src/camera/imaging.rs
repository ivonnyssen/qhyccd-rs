
use eyre::{eyre, Result};

use crate::backend::{read_lock, CameraBackend};
use crate::{ImageData, QHYError::*};

#[cfg(feature = "simulation")]
use crate::simulation;

#[cfg(not(test))]
use libqhyccd_sys::{
    BeginQHYCCDLive, CancelQHYCCDExposing, CancelQHYCCDExposingAndReadout, ExpQHYCCDSingleFrame,
    GetQHYCCDExposureRemaining, GetQHYCCDLiveFrame, GetQHYCCDMemLength, GetQHYCCDSingleFrame,
    StopQHYCCDLive, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    BeginQHYCCDLive, CancelQHYCCDExposing, CancelQHYCCDExposingAndReadout, ExpQHYCCDSingleFrame,
    GetQHYCCDExposureRemaining, GetQHYCCDLiveFrame, GetQHYCCDMemLength, GetQHYCCDSingleFrame,
    StopQHYCCDLive, QHYCCD_ERROR, QHYCCD_SUCCESS,
};

use super::Camera;

impl Camera {
    /// Starts Live Video Mode on the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.begin_live().expect("begin_live failed");
    /// ```
    pub fn begin_live(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, BeginLiveError { error_code: 0 })?;
                match unsafe { BeginQHYCCDLive(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = BeginLiveError { error_code };
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
                state.live_mode_active = true;
                Ok(())
            }
        }
    }

    /// Stops Live Video Mode on the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.begin_live().expect("begin_live failed");
    /// camera.end_live().expect("end_live failed");
    /// ```
    pub fn end_live(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, EndLiveError { error_code: 0 })?;
                match unsafe { StopQHYCCDLive(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = EndLiveError { error_code };
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
                state.live_mode_active = false;
                Ok(())
            }
        }
    }

    /// Returns the number of bytes needed to retrieve the image stored in the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let image_size = camera.get_image_size().expect("get_image_size failed");
    /// println!("Image size: {}", image_size);
    /// ```
    pub fn get_image_size(&self) -> Result<usize> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetImageSizeError)?;
                match unsafe { GetQHYCCDMemLength(handle) } {
                    QHYCCD_ERROR => {
                        let error = GetImageSizeError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    size => Ok(size as usize),
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
                Ok(state.calculate_buffer_size())
            }
        }
    }

    /// Returns the image stored in the camera as `ImageData` struct if the camera is in Live Video Mode
    /// # Example
    /// ```no_run
    /// use std::{thread, time::Duration};
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.begin_live().expect("begin_live failed");
    /// thread::sleep(Duration::from_millis(100));
    /// let buffer_size = camera.get_image_size().expect("get_image_size failed");
    /// let image = camera.get_live_frame(buffer_size).expect("get_live_frame failed");
    /// println!("Image: {:?}", image);
    /// ```
    pub fn get_live_frame(&self, buffer_size: usize) -> Result<ImageData> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetLiveFrameError { error_code: 0 })?;
                let mut width: u32 = 0;
                let mut height: u32 = 0;
                let mut bpp: u32 = 0;
                let mut channels: u32 = 0;
                let mut buffer = vec![0u8; buffer_size];
                match unsafe {
                    GetQHYCCDLiveFrame(
                        handle,
                        &mut width as *mut u32,
                        &mut height as *mut u32,
                        &mut bpp as *mut u32,
                        &mut channels as *mut u32,
                        buffer.as_mut_ptr(),
                    )
                } {
                    QHYCCD_SUCCESS => Ok(ImageData {
                        data: buffer,
                        width,
                        height,
                        bits_per_pixel: bpp,
                        channels,
                    }),
                    error_code => {
                        let error = GetLiveFrameError { error_code };
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
                if !state.live_mode_active {
                    return Err(eyre!(GetLiveFrameError {
                        error_code: QHYCCD_ERROR
                    }));
                }

                let (width, height) = state.get_current_image_dimensions();
                let bpp = state.bit_depth;
                let channels = state.get_channels();

                let generator = simulation::ImageGenerator::default();
                let data = if bpp <= 8 {
                    generator.generate_8bit(width, height, channels)
                } else {
                    generator.generate_16bit(width, height, channels)
                };

                Ok(ImageData {
                    data,
                    width,
                    height,
                    bits_per_pixel: bpp,
                    channels,
                })
            }
        }
    }

    /// Returns the image stored in the camera as `ImageData` struct if the camera is in Single Frame Mode
    /// # Example
    ///
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_camera_single_frame_exposure failed");
    /// let buffer_size = camera.get_image_size().expect("get_camera_image_size failed");
    /// let image = camera.get_single_frame(buffer_size).expect("get_camera_single_frame failed");
    /// ```
    pub fn get_single_frame(&self, buffer_size: usize) -> Result<ImageData> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetSingleFrameError { error_code: 0 })?;
                let mut width: u32 = 0;
                let mut height: u32 = 0;
                let mut bpp: u32 = 0;
                let mut channels: u32 = 0;
                let mut buffer = vec![0u8; buffer_size];
                match unsafe {
                    GetQHYCCDSingleFrame(
                        handle,
                        &mut width as *mut u32,
                        &mut height as *mut u32,
                        &mut bpp as *mut u32,
                        &mut channels as *mut u32,
                        buffer.as_mut_ptr(),
                    )
                } {
                    QHYCCD_SUCCESS => Ok(ImageData {
                        data: buffer,
                        width,
                        height,
                        bits_per_pixel: bpp,
                        channels,
                    }),
                    error_code => {
                        let error = GetSingleFrameError { error_code };
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

                // Wait for exposure to complete if one is in progress
                if state.exposure_start.is_some() && !state.is_exposure_complete() {
                    // In real camera, this would block. For simulation, we just wait.
                    while !state.is_exposure_complete() {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }

                let (width, height) = state.get_current_image_dimensions();
                let bpp = state.bit_depth;
                let channels = state.get_channels();

                let generator = simulation::ImageGenerator::default();
                let data = if bpp <= 8 {
                    generator.generate_8bit(width, height, channels)
                } else {
                    generator.generate_16bit(width, height, channels)
                };

                // Clear exposure state
                state.exposure_start = None;

                Ok(ImageData {
                    data,
                    width,
                    height,
                    bits_per_pixel: bpp,
                    channels,
                })
            }
        }
    }

    /// Start a long exposure
    /// Make sure to set the exposure time before calling this function
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// ```
    pub fn start_single_frame_exposure(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, StartSingleFrameExposureError { error_code: 0 })?;
                match unsafe { ExpQHYCCDSingleFrame(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = StartSingleFrameExposureError { error_code };
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
                state.start_exposure();
                Ok(())
            }
        }
    }

    /// Gets the remaining exposure time
    /// it needs to be called from a different thread than the one that called `start_single_frame_exposure`
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// let remaining = camera.get_remaining_exposure_us().expect("get_remaining_exposure_us failed");
    /// println!("Remaining exposure time: {}", remaining);
    /// ```
    pub fn get_remaining_exposure_us(&self) -> Result<u32> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetExposureRemainingError)?;
                match unsafe { GetQHYCCDExposureRemaining(handle) } {
                    QHYCCD_ERROR => {
                        let error = GetExposureRemainingError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    remaining if { remaining <= 100 } => Ok(0),
                    remaining => Ok(remaining),
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
                Ok(state.get_remaining_exposure_us())
            }
        }
    }

    /// Stops the current exposure
    /// the image data stays in the camera and must be retrieved with `get_single_frame`
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// camera.stop_exposure().expect("stop_exposure failed");
    /// ```
    pub fn stop_exposure(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, StopExposureError { error_code: 0 })?;
                match unsafe { CancelQHYCCDExposing(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = StopExposureError { error_code };
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
                state.cancel_exposure();
                Ok(())
            }
        }
    }

    /// Stops the current exposure and discards the image data in the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// camera.abort_exposure_and_readout().expect("abort_exposure_and_readout failed");
    /// ```
    pub fn abort_exposure_and_readout(&self) -> Result<()> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, AbortExposureAndReadoutError { error_code: 0 })?;
                match unsafe { CancelQHYCCDExposingAndReadout(handle) } {
                    QHYCCD_SUCCESS => Ok(()),
                    error_code => {
                        let error = AbortExposureAndReadoutError { error_code };
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
                state.cancel_exposure();
                Ok(())
            }
        }
    }
}
