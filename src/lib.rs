//! # QHYCCD SDK bindings for Rust
//!
//! This crate provides a safe interface to the QHYCCD SDK for controlling QHYCCD cameras, filter wheels and focusers.
//! The libqhyccd-sys crate provides the raw FFI bindings. It uses tracing for logging and eyre for error handling.
//!
//! # Example
//! ```no_run
//! use qhyccd_rs::Sdk;
//! let sdk = Sdk::new().expect("SDK::new failed");
//! let sdk_version = sdk.version().expect("get_sdk_version failed");
//! println!("SDK version: {:?}", sdk_version);
//! ```
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::ffi::{c_char, CStr};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use eyre::{eyre, Result, WrapErr};
use tracing::error;

use crate::QHYError::*;
#[macro_use]
extern crate educe;

#[cfg(test)]
pub mod mocks;

#[cfg(feature = "simulation")]
pub mod simulation;

#[cfg(feature = "simulation")]
use simulation::SimulatedCameraState;

#[cfg(not(test))]
use libqhyccd_sys::{
    BeginQHYCCDLive, CancelQHYCCDExposing, CancelQHYCCDExposingAndReadout, CloseQHYCCD,
    ExpQHYCCDSingleFrame, GetQHYCCDChipInfo, GetQHYCCDEffectiveArea, GetQHYCCDExposureRemaining,
    GetQHYCCDFWVersion, GetQHYCCDId, GetQHYCCDLiveFrame, GetQHYCCDMemLength, GetQHYCCDModel,
    GetQHYCCDNumberOfReadModes, GetQHYCCDOverScanArea, GetQHYCCDParam, GetQHYCCDParamMinMaxStep,
    GetQHYCCDReadMode, GetQHYCCDReadModeName, GetQHYCCDReadModeResolution, GetQHYCCDSDKVersion,
    GetQHYCCDSingleFrame, GetQHYCCDType, InitQHYCCD, InitQHYCCDResource, IsQHYCCDCFWPlugged,
    IsQHYCCDControlAvailable, OpenQHYCCD, ReleaseQHYCCDResource, ScanQHYCCD, SetQHYCCDBinMode,
    SetQHYCCDBitsMode, SetQHYCCDDebayerOnOff, SetQHYCCDParam, SetQHYCCDReadMode,
    SetQHYCCDResolution, SetQHYCCDStreamMode, StopQHYCCDLive, QHYCCD_ERROR, QHYCCD_ERROR_F64,
    QHYCCD_SUCCESS,
};

#[cfg(test)]
use crate::mocks::mock_libqhyccd_sys::{
    BeginQHYCCDLive, CancelQHYCCDExposing, CancelQHYCCDExposingAndReadout, CloseQHYCCD,
    ExpQHYCCDSingleFrame, GetQHYCCDChipInfo, GetQHYCCDEffectiveArea, GetQHYCCDExposureRemaining,
    GetQHYCCDFWVersion, GetQHYCCDId, GetQHYCCDLiveFrame, GetQHYCCDMemLength, GetQHYCCDModel,
    GetQHYCCDNumberOfReadModes, GetQHYCCDOverScanArea, GetQHYCCDParam, GetQHYCCDParamMinMaxStep,
    GetQHYCCDReadMode, GetQHYCCDReadModeName, GetQHYCCDReadModeResolution, GetQHYCCDSDKVersion,
    GetQHYCCDSingleFrame, GetQHYCCDType, InitQHYCCD, InitQHYCCDResource, IsQHYCCDCFWPlugged,
    IsQHYCCDControlAvailable, OpenQHYCCD, ReleaseQHYCCDResource, ScanQHYCCD, SetQHYCCDBinMode,
    SetQHYCCDBitsMode, SetQHYCCDDebayerOnOff, SetQHYCCDParam, SetQHYCCDReadMode,
    SetQHYCCDResolution, SetQHYCCDStreamMode, StopQHYCCDLive, QHYCCD_ERROR, QHYCCD_ERROR_F64,
    QHYCCD_SUCCESS,
};

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur when interacting with the QHYCCD SDK
/// most functions from the SDK return `u32::MAX` on error
/// where it is different, is is noted in the documentation
#[allow(missing_docs)]
pub enum QHYError {
    #[error("Error initializing QHYCCD SDK, error code {}", error_code)]
    InitSDKError { error_code: u32 },
    #[error("Error closing QHYCCD SDK, error code {}", error_code)]
    CloseSDKError { error_code: u32 },
    #[error("Error getting QHYCCD SDK version, error code {:?}", error_code)]
    GetSDKVersionError { error_code: u32 },
    #[error("Error scanning QHYCCD cameras")]
    ScanQHYCCDError,
    #[error("Error opening camera")]
    OpenCameraError,
    #[error("Error camera id, error code {:?}", error_code)]
    GetCameraIdError { error_code: u32 },
    #[error("Error getting firmware version, error code {:?}", error_code)]
    GetFirmwareVersionError { error_code: u32 },
    #[error("Error setting camera read mode, error code {:?}", error_code)]
    SetReadoutModeError { error_code: u32 },
    #[error("Error setting camera stream mode, error code {:?}", error_code)]
    SetStreamModeError { error_code: u32 },
    #[error("Error initializing camera {:?}", error_code)]
    InitCameraError { error_code: u32 },
    #[error("Error getting camera CCD info, error code {:?}", error_code)]
    GetCCDInfoError { error_code: u32 },
    #[error("Error setting camera bit mode, error code {:?}", error_code)]
    SetBitModeError { error_code: u32 },
    #[error("Error setting camera debayer on/off, error code {:?}", error_code)]
    SetDebayerError { error_code: u32 },
    #[error("Error setting camera bin mode, error code {:?}", error_code)]
    SetBinModeError { error_code: u32 },
    #[error("Error setting camera sub frame, error code {:?}", error_code)]
    SetRoiError { error_code: u32 },
    #[error("Error getting camera parameter, error code {:?}", control)]
    GetParameterError {
        /// here the control field has the `Control` enum variant we tried to get the value for
        control: Control,
    },
    #[error("Error setting camera parameter, error code {:?}", error_code)]
    SetParameterError { error_code: u32 },
    #[error("Error starting camera live mode, error code {:?}", error_code)]
    BeginLiveError { error_code: u32 },
    #[error("Error stopping camera live mode, error code {:?}", error_code)]
    EndLiveError { error_code: u32 },
    #[error("Error getting image size, error code")]
    GetImageSizeError,
    #[error("Error getting camera live frame, error code {:?}", error_code)]
    GetLiveFrameError { error_code: u32 },
    #[error("Error getting camera single frame, error code {:?}", error_code)]
    GetSingleFrameError { error_code: u32 },
    #[error("Error closing camera, error code {:?}", error_code)]
    CloseCameraError { error_code: u32 },
    #[error("Error getting camera overscan area, error code {:?}", error_code)]
    GetOverscanAreaError { error_code: u32 },
    #[error("Error getting camera effective area, error code {:?}", error_code)]
    GetEffectiveAreaError { error_code: u32 },
    #[error("Error getting determining support for camera feature {:?}", control)]
    IsControlAvailableError { control: Control },
    #[error("Error starting single frame exposure, error code {:?}", error_code)]
    StartSingleFrameExposureError { error_code: u32 },
    #[error("Error getting camera number of read modes")]
    GetNumberOfReadoutModesError,
    #[error("Error getting camera read mode name")]
    GetReadoutModeNameError,
    #[error("Error getting camera read mode resolution")]
    GetReadoutModeResolutionError,
    #[error("Error getting camera readout mode")]
    GetReadoutModeError,
    #[error("Error getting model of camera {:?}", error_code)]
    GetCameraModelError { error_code: u32 },
    #[error("Error getting type of camera")]
    GetCameraTypeError,
    #[error("Error getting remaining exposure time")]
    GetExposureRemainingError,
    #[error("Error stopping exposure {:?}", error_code)]
    StopExposureError { error_code: u32 },
    #[error("Error canceling exposure and readout {:?}", error_code)]
    AbortExposureAndReadoutError { error_code: u32 },
    #[error("Error getting camera CFW plugged status")]
    IsCfwPluggedInError,
    #[error("Error camera is not open")]
    CameraNotOpenError,
    #[error(
        "Error getting camera min, max, step for parameter, error code {:?}",
        control
    )]
    GetMinMaxStepError {
        /// here the control field has the `Control` enum variant we tried to get the value for
        control: Control,
    },
    #[error("Error getting filter wheel position")]
    GetCfwPositionError,
    #[error("Error setting filter wheel position")]
    SetCfwPositionError,
    #[error("Error opening the filter wheel")]
    OpenFilterWheelError,
    #[error("Error closing the filter wheel error code {:?}", error_code)]
    CloseFilterWheelError { error_code: u32 },
    #[error("Error getting the number of filters")]
    GetNumberOfFiltersError,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// Controls used in `is_control_available` and `set_parameter` nad `get_parameter`
/// documentation is taken from the QHYCCD SDK
/// here <https://www.qhyccd.cn/file/repository/publish/SDK/code/QHYCCD%20SDK_API_EN_V2.3.pdf>
pub enum Control {
    /// Check if support brightness
    Brightness = 0,
    /// Check if support contrast
    Contrast = 1,
    /// Check if support red balance
    Wbr = 2,
    /// Check if support blue balance
    Wbb = 3,
    /// Check if support green balance
    Wbg = 4,
    /// Check if support gamma
    Gamma = 5,
    /// Check if support gain
    Gain = 6,
    /// Check if support offset
    Offset = 7,
    /// Used to set exposure time in microseconds
    Exposure = 8,
    /// Check if support speed
    Speed = 9,
    /// Check if support bits setting
    TransferBit = 10,
    /// Check if support get channels number(Discontinued)
    Channels = 11,
    /// Check if support traffic
    UsbTraffic = 12,
    /// Check if support row denoise
    RowDeNoise = 13,
    /// Check if support get current temperature
    CurTemp = 14,
    /// Check if support get current PWM
    CurPWM = 15,
    /// Check if support manual cool mode
    ManualPWM = 16,
    /// Check if support CFW - Color Filter Wheel
    CfwPort = 17,
    /// Check if support auto cool mode
    Cooler = 18,
    /// Check if support ST4 port
    St4Port = 19,
    /// Check if support get bayer matrix - clashes with `CamIsColor`, but use this one here
    CamColor = 20,
    /// Check if support 1X1 bin mode
    CamBin1x1mode = 21,
    /// Check if support 2X2 bin mode
    CamBin2x2mode = 22,
    /// Check if support 3X3 bin mode
    CamBin3x3mode = 23,
    /// Check if support 4X4 bin mode
    CamBin4x4mode = 24,
    /// Check if support machine shutter
    CamMechanicalShutter = 25,
    /// Check if support trigger mode
    CamTrigerInterface = 26,
    /// Check if support temperature over protect,this
    /// function will limit cooler max PWM be 70%(Disabled)
    CamTecoverprotectInterface = 27,
    /// Check whether the camera supports the
    /// SINGNALCLAMP function, which is a unique feature
    /// of CCD cameras for dark bands behind bright stars
    CamSignalClampInterface = 28,
    /// Check whether the camera supports fine tuning,
    /// which is used for CCD cameras to optimize the noise
    /// characteristics of the camera by fine-tuning the CCD
    /// drive and sampling timing
    CamFinetoneInterface = 29,
    /// Check whether the camera supports shutter motor
    /// heating
    CamShutterMotorHeatingInterface = 30,
    /// Check whether the camera supports FPN calibration,
    /// which reduces FPN noise such as vertical stripes
    CamCalibrateFpnInterface = 31,
    /// Check whether the camera supports an on-chip
    /// temperature sensor
    CamChipTemperatureSensorInterface = 32,
    /// Check whether the camera supports the USB
    /// minimum speed readout function (this function
    /// duplicates the CONTROL_SPEED function and is no
    /// longer in use)
    CamUsbReadoutSlowestInterface = 33,
    /// Check whether the camera supports 8-bit image data
    /// output
    Cam8bits = 34,
    /// Check whether the camera supports 16-bit image
    /// data output
    Cam16bits = 35,
    /// Check whether the camera supports GPS
    CamGps = 36,
    /// Check whether the camera supports the function of
    /// overscanning area calibration
    CamIgnoreOverscanInterface = 37,
    // Check whether the camera supports automatic white
    // balance
    //Qhyccd3aAutoWhiteBalance = 38,
    /// Check whether the camera supports auto exposure
    Qhyccd3aAutoexposure = 39,
    /// Check whether the camera supports autofocus
    Qhyccd3aAutofocus = 40,
    /// Check whether the camera supports glow
    /// suppression
    Ampv = 41,
    /// Check whether the camera supports WDM broadcast
    Vcam = 42,
    /// Check whether preview mode is supported (not
    /// enabled)
    CamViewMode = 43,
    /// Check whether the camera can obtain the number of
    /// filter wheel holes
    CfwSlotsNum = 44,
    /// Check whether the camera is exposed (not enabled)
    IsExposingDone = 45,
    /// Check whether the camera can be stretched Black
    /// gray scale
    ScreenStretchB = 46,
    /// Check whether the camera can White grayscale
    /// stretching
    ScreenStretchW = 47,
    /// Check whether the camera supports DDR
    DDR = 48,
    /// Check whether the camera supports the high-low
    /// gain switching function
    CamLightPerformanceMode = 49,
    ///C heck if the camera is a 5II series camera that
    /// supports guide mode
    CamQhy5IIGuideMode = 50,
    /// Check whether the camera can get the current
    /// amount of DDR buffer data
    DDRBufferCapacity = 51,
    /// Check whether the camera can get the buffer read
    /// threshold
    DDRBufferReadThreshold = 52,
    /// Check whether the camera can obtain the default
    /// gain recommendation
    DefaultGain = 53,
    /// Check whether the camera can obtain the default
    /// bias recommendation
    DefaultOffset = 54,
    /// Check whether the camera can get the actual bits of
    /// output data
    OutputDataActualBits = 55,
    /// Check whether the camera supports getting output
    /// data alignment formats
    OutputDataAlignment = 56,
    /// Check whether the camera supports single frame
    /// mode
    CamSingleFrameMode = 57,
    /// Check whether the camera supports live frame mode
    CamLiveVideoMode = 58,
    /// Check if the camera is color
    CamIsColor = 59,
    /// Check whether the camera supports hardware frame
    /// counting
    HasHardwareFrameCounter = 60,
    /// Get the maximum value of CONTROL_ID (deprecated)
    MaxIdError = 61,
    /// Check whether the camera supports a humidity
    /// sensor
    CamHumidity = 62,
    /// Check whether the camera supports pressure sensors
    CamPressure = 63,
    /// Check whether the camera supports vacuum pump
    VacuumPump = 64,
    /// Check that the camera supports internal circulation
    /// pumps
    SensorChamberCyclePump = 65,
    /// Check whether the camera supports 32-bit image
    /// data output
    Cam32bits = 66,
    /// Check whether the camera supports ULVO status
    /// detection
    CamSensorUlvoStatus = 67,
    /// Check whether the camera supports phase
    /// adjustment, which handles image streaks due to
    /// phase
    CamSensorPhaseReTrain = 68,
    /// Check whether the camera supports Flash read and
    /// write Config
    CamInitConfigFromFlash = 69,
    /// Check whether the camera supports multiple trigger
    /// mode Settings
    CamTriggerMode = 70,
    /// Check whether the camera supports trigger output
    CamTriggerOut = 71,
    /// Check whether the camera supports Burst mode
    CamBurstMode = 72,
    /// Check whether the camera supports the signal lamp
    /// function (currently only for customized models
    CamSpeakerLedAlarm = 73,
    /// Check whether camera FPGA supports watchdog
    /// processing function (currently only for customized
    /// models)
    CamWatchDogFpga = 74,
    /// Check whether the camera supports 6X6 BIN
    CamBin6x6mode = 75,
    /// Check whether the camera supports 8X8 BIN
    CamBin8x8mode = 76,
    /// Check whether the camera sensor supports global
    /// LED calibration lights
    CamGlobalSensorGpsLED = 77,
    /// Check whether the camera supports image
    /// processing
    ImgProc = 78,
    /// not documented
    RemoveRbi = 79,
    /// not documented
    GlobalReset = 80,
    /// not documented
    FrameDetect = 81,
    /// not documented
    CamGainDbConversion = 82,
    /// not documented
    CamCurveSystemGain = 83,
    /// not documented
    CamCurveFullWell = 84,
    /// not documented
    CamCurveReadoutNoise = 85,
    /// not documented
    MaxId = 86,
    /// not documented - see missing value 38
    Autowhitebalance = 1024,
    /// not documented
    Autoexposure = 1025,
    /// not documented
    AutoexpMessureValue = 1026,
    /// not documented
    AutoexpMessureMethod = 1027,
    /// not documented
    ImageStabilization = 1028,
    /// not documented
    GaindB = 1029,
}

#[derive(Debug, PartialEq)]
/// Stream mode used in `set_stream_mode`
pub enum StreamMode {
    /// Long exposure mode
    SingleFrameMode = 0,
    /// Live video mode
    LiveMode = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Camera sensor info
pub struct CCDChipInfo {
    /// chip width in um
    pub chip_width: f64,
    /// chip height in um
    pub chip_height: f64,
    /// number of horizontal pixels
    pub image_width: u32,
    /// number of vertical pixels
    pub image_height: u32,
    /// pixel width in um
    pub pixel_width: f64,
    /// pixel height in um
    pub pixel_height: f64,
    /// maximum bit depth for transfer
    pub bits_per_pixel: u32,
}

#[derive(Debug, PartialEq)]
/// the image data coming from the camera in `get_live_frame` and `get_single_frame`
pub struct ImageData {
    /// the image data
    pub data: Vec<u8>,
    /// the width of the image in pixels
    pub width: u32,
    /// the height of the image in pixels
    pub height: u32,
    /// the number of bits per pixel
    pub bits_per_pixel: u32,
    /// the number of channels 1 or 4 most of the time
    pub channels: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// this struct is used in `get_overscan_area`, `get_effective_area`, `set_roi` and `get_roi`
pub struct CCDChipArea {
    /// the x coordinate of the top left corner of the area
    pub start_x: u32,
    /// the y coordinate of the top left corner of the area
    pub start_y: u32,
    /// the width of the area in pixels
    pub width: u32,
    /// the height of the area in pixels
    pub height: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
/// this struct is returned from `is_control_available` when used with `Control::CamColor`
pub enum BayerMode {
    GBRG = 1,
    GRBG = 2,
    BGGR = 3,
    RGGB = 4,
}

impl TryFrom<u32> for BayerMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == BayerMode::GBRG as u32 => Ok(BayerMode::GBRG),
            x if x == BayerMode::GRBG as u32 => Ok(BayerMode::GRBG),
            x if x == BayerMode::BGGR as u32 => Ok(BayerMode::BGGR),
            x if x == BayerMode::RGGB as u32 => Ok(BayerMode::RGGB),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
/// used to store readout mode numbers and their descriptions coming from `get_readout_mode_name`
pub struct ReadoutMode {
    /// the number of the mode staring with 0
    pub id: u32,
    /// the name of the mode e.g., `"STANDARD MODE"`
    pub name: String,
}

#[derive(Debug, PartialEq)]
/// returned from `SDK::version`
pub struct SDKVersion {
    /// the year of the SDK version
    pub year: u32,
    /// the month of the SDK version
    pub month: u32,
    /// the day of the SDK version
    pub day: u32,
    /// the subday of the SDK version
    pub subday: u32,
}
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

#[derive(Debug, PartialEq, Copy, Clone)]
struct QHYCCDHandle {
    pub ptr: *const std::ffi::c_void,
}

//Safety: QHYCCDHandle is only used in Camera and Camera is Send and Sync
unsafe impl Send for QHYCCDHandle {}
unsafe impl Sync for QHYCCDHandle {}

/// Internal backend for camera operations
#[derive(Debug)]
enum CameraBackend {
    /// Real hardware camera using FFI calls
    Real {
        handle: Arc<RwLock<Option<QHYCCDHandle>>>,
    },
    /// Simulated camera for testing
    #[cfg(feature = "simulation")]
    Simulated {
        state: Arc<RwLock<SimulatedCameraState>>,
    },
}

impl Clone for CameraBackend {
    fn clone(&self) -> Self {
        match self {
            CameraBackend::Real { handle } => CameraBackend::Real {
                handle: Arc::clone(handle),
            },
            #[cfg(feature = "simulation")]
            CameraBackend::Simulated { state } => CameraBackend::Simulated {
                state: Arc::clone(state),
            },
        }
    }
}

impl PartialEq for CameraBackend {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CameraBackend::Real { .. }, CameraBackend::Real { .. }) => true,
            #[cfg(feature = "simulation")]
            (CameraBackend::Simulated { .. }, CameraBackend::Simulated { .. }) => true,
            #[allow(unreachable_patterns)]
            _ => false,
        }
    }
}

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq)]
/// The representation of a camera. It is constructed by the SDK and can be used to
/// interact with the camera.
pub struct Camera {
    id: String,
    #[educe(PartialEq(ignore))]
    backend: CameraBackend,
}

macro_rules! read_lock {
    ($var:expr, $wrap:expr) => {
        $var.read().map_err(|err| {
            tracing::error!(error=?err);
            eyre!("Could not acquire read lock on camera handle")
        }).and_then(|lock|{match *lock {
            Some(handle) => Ok(handle.ptr),
            None => {
                tracing::error!(error = ?CameraNotOpenError);
                Err(eyre!(CameraNotOpenError))
            }
        }}).wrap_err($wrap)
    }
}

#[allow(unused_unsafe)]
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

    /// Sets the stream mode of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, StreamMode};
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
    /// use qhyccd_rs::{Sdk};
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

    /// retunrs the model or short description of the camera - this does not work for all cameras
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
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

    /// returns the firmware version of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
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

    /// Returns the number of readout modes of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// println!("Number of readout modes: {}", num_readout_modes);
    /// ```
    pub fn get_number_of_readout_modes(&self) -> Result<u32> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetNumberOfReadoutModesError)?;

                let mut num: u32 = 0;
                match unsafe { GetQHYCCDNumberOfReadModes(handle, &mut num as *mut u32) } {
                    QHYCCD_ERROR => {
                        let error = GetNumberOfReadoutModesError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    _ => Ok(num),
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
                Ok(state.config.readout_modes.len() as u32)
            }
        }
    }

    /// Returns the readout mode name with the given index. Make sure to check the number of readout modes.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// for index in 0..num_readout_modes {
    ///    let readout_mode_name = camera.get_readout_mode_name(index).expect("get_readout_mode_name failed");
    ///   println!("Readout mode {}: {}", index, readout_mode_name);
    /// }
    /// ```
    pub fn get_readout_mode_name(&self, index: u32) -> Result<String> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetReadoutModeNameError)?;
                let mut name: [c_char; 80] = [0; 80];
                match unsafe { GetQHYCCDReadModeName(handle, index, name.as_mut_ptr()) } {
                    QHYCCD_ERROR => {
                        let error = GetReadoutModeNameError;
                        tracing::error!(error = ?error);
                        Err(eyre!(error))
                    }
                    _ => {
                        let name = match unsafe { CStr::from_ptr(name.as_ptr()) }.to_str() {
                            Ok(name) => name,
                            Err(error) => {
                                tracing::error!(error = ?error);
                                return Err(eyre!(error));
                            }
                        };
                        Ok(name.to_string())
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
                    .readout_modes
                    .get(index as usize)
                    .map(|(name, _)| name.clone())
                    .ok_or_else(|| eyre!(GetReadoutModeNameError))
            }
        }
    }

    /// Returns the resolution of the readout mode with the given index. Make sure to check the number of readout modes.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// for index in 0..num_readout_modes {
    ///   let readout_mode_resolution = camera.get_readout_mode_resolution(index).expect("get_readout_mode_resolution failed");
    ///  println!("Readout mode {}: {:?}", index, readout_mode_resolution);
    /// }
    /// ```
    pub fn get_readout_mode_resolution(&self, index: u32) -> Result<(u32, u32)> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetReadoutModeResolutionError)?;

                let mut width: u32 = 0;
                let mut height: u32 = 0;
                match unsafe {
                    GetQHYCCDReadModeResolution(
                        handle,
                        index,
                        &mut width as *mut u32,
                        &mut height as *mut u32,
                    )
                } {
                    QHYCCD_SUCCESS => Ok((width, height)),
                    _ => {
                        let error = GetReadoutModeResolutionError;
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
                    .readout_modes
                    .get(index as usize)
                    .map(|(_, res)| *res)
                    .ok_or_else(|| eyre!(GetReadoutModeResolutionError))
            }
        }
    }

    /// Returns the current readout mode of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let readout_mode = camera.get_readout_mode().expect("get_readout_mode failed");
    /// println!("Readout mode: {}", readout_mode);
    /// ```
    pub fn get_readout_mode(&self) -> Result<u32> {
        match &self.backend {
            CameraBackend::Real { handle } => {
                let handle = read_lock!(handle, GetReadoutModeError)?;
                let mut mode: u32 = 0;
                match unsafe { GetQHYCCDReadMode(handle, &mut mode as *mut u32) } {
                    QHYCCD_SUCCESS => Ok(mode),
                    _ => {
                        let error = GetReadoutModeError;
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
                Ok(state.readout_mode)
            }
        }
    }

    /// Not sure what this does, for a QHY178M Cool it returns 4010
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let tipe = camera.get_type().expect("get_type failed");
    /// println!("Type: {}", tipe);
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

    /// Sets the binning mode of the camera
    /// Only symmetric binnings are supported
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_bin_mode(2, 2).expect("set_bin_mode failed");
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
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_debayer(false).expect("set_debayer failed");
    ///```
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
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// use qhyccd_rs::CCDChipArea;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let roi = CCDChipArea {
    ///     start_x: 0,
    ///     start_y: 0,
    ///     width: 1000,
    ///     height: 1000,
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
    /// /* Download images in between */
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
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// // wait for exposure to finish
    /// let buffer_size = camera.get_image_size().expect("get_camera_image_size failed");
    /// let image = camera.get_single_frame(buffer_size).expect("get_camera_single_frame failed");
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
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.begin_live().expect("begin_live failed");
    /// let size = camera.get_image_size().expect("get_camera_image_size failed");
    /// for _ in 0..1000 {
    ///     let result = camera.get_live_frame(size);
    ///     if result.is_err() {
    ///         thread::sleep(Duration::from_millis(100));
    ///         continue;
    ///     }
    ///     let image = result.unwrap();
    ///    /* Do something with the image */
    ///     break;
    /// }
    /// camera.end_live().expect("end_camera_live failed");
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

    /// Get the chip area including overscan area
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,CCDChipArea};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let chip_area = camera.get_overscan_area().expect("get_overscan_area failed");
    /// println!("Chip area: {:?}", chip_area);
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
    /// use qhyccd_rs::{Sdk,Camera,CCDChipArea};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let chip_area = camera.get_effective_area().expect("get_overscan_area failed");
    /// println!("Chip area: {:?}", chip_area);
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

    /// Start a long exposure
    /// Make sure to set the exposure time before calling this function
    /// this function blocks the current thread and only returns when the exposure is finished
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_param failed"); // this is in micro seconds
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
    /// use qhyccd_rs::{Sdk,Camera};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// /* start exposure on a different thread*/
    /// let remaining_exposure = camera.get_remaining_exposure_us().expect("get_remaining_exposure_us failed");
    /// println!("Remaining exposure: {}", remaining_exposure);
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
    /// use qhyccd_rs::{Sdk,Camera};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// /* start exposure on a different thread*/
    /// camera.stop_exposure().expect("stop_exposure failed");
    /// /* retrieve image data */
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
    /// use qhyccd_rs::{Sdk,Camera};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// /* start exposure on a different thread*/
    /// camera.abort_exposure_and_readout().expect("abort_exposure failed");
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

    /// Returns information about the control given to the function
    /// # Returns
    /// `Err` if the control is not available
    /// `Ok(info: u32)` if the control is available. Info here is different for controls that have non boolean answers
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// if camera.is_control_available(Control::CamLiveVideoMode).is_none()
    /// {
    ///    println!("Control::CamLiveVideoMode is not supported");
    /// }
    /// let camera_is_color = camera.is_control_available(Control::CamColor).is_some(); //this returns a `BayerID` if it is a color camera
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

    /// Returns information about the chip in the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,CCDChipInfo};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let chip_info = camera.get_ccd_info().expect("get_ccd_info failed");
    /// println!("Chip info: {:?}", chip_info);
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

    /// Sets the USB transfer mode to either 8 or 16 bit
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// camera.set_bit_mode(8).expect("set_bit_mode failed");
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

    /// Returns the value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.open().expect("open failed");
    /// let number_of_filter_wheel_positions = match camera.is_control_available(Control::CfwSlotsNum).is_some() {
    ///     true => camera.get_parameter(Control::CfwSlotsNum).unwrap_or_default() as u32,
    ///     false => 0,
    /// };
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

unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}

#[derive(Educe)]
#[educe(Debug, Clone, PartialEq)]
/// The representation of a filter wheel. It is constructed by the SDK and can be used to
/// interact with the filter wheel - every filter wheel is always plugged into a camera.
pub struct FilterWheel {
    camera: Camera,
}

/// Filter wheels are directly connected to the QHY camera and can be controlled through the camera
#[allow(unused_unsafe)]
impl FilterWheel {
    /// Creates a new instance of the filter wheel. The Sdk automatically finds all filter wheels and provides them in it's `filter_wheels()` iterator. Creating
    /// a filter wheek manually should only be needed for rare cases.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, Camera, FilterWheel};
    /// let fw = FilterWheel::new(Camera::new("filter wheel id from sdk".to_string()));
    /// println!("FilterWheel: {:?}", fw);
    /// ```
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    /// Returns the id of the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// println!("Filter wheel id: {}", fw.id());
    /// ```
    pub fn id(&self) -> &str {
        self.camera.id()
    }

    /// Opens the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// ```
    pub fn open(&self) -> Result<()> {
        self.camera.open()
    }

    /// Returns `true` if the filter wheel is open
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let is_open = fw.is_open();
    /// println!("Is filter wheel open: {:?}", is_open);
    /// ```
    pub fn is_open(&self) -> Result<bool> {
        self.camera.is_open()
    }

    /// Returns `true` if the filter wheel is plugged into the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let is_cfw_plugged_in = fw.is_cfw_plugged_in().expect("is_cfw_plugged_in failed");
    /// println!("Is filter wheel plugged in: {}", is_cfw_plugged_in);
    /// ```
    pub fn is_cfw_plugged_in(&self) -> Result<bool> {
        self.camera.is_cfw_plugged_in()
    }

    /// Closes the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// fw.close().expect("close failed");
    /// ```
    pub fn close(&self) -> Result<()> {
        self.camera.close()
    }

    /// Returns the number of filters in the filter wheel
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let number_of_filters = fw.get_number_of_filters().expect("get_number_of_filters failed");
    /// println!("Number of filters: {}", number_of_filters);
    /// ```
    pub fn get_number_of_filters(&self) -> Result<u32> {
        match self.camera.is_control_available(Control::CfwSlotsNum) {
            Some(_) => self.camera.get_parameter(Control::CfwSlotsNum).map_or_else(
                |e| {
                    error!(?e, "could not get number of filters from camera");
                    Err(e)
                },
                |num| Ok(num as u32),
            ),
            None => {
                tracing::debug!("I'm a filter wheel without filters. :(");
                Err(eyre!(GetNumberOfFiltersError))
            }
        }
    }

    /// Returns the current filter wheel position
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// let current_position = fw.get_fw_position().expect("get_fw_position failed");
    /// println!("Current position: {}", current_position);
    /// ```
    pub fn get_fw_position(&self) -> Result<u32> {
        match self.camera.is_control_available(Control::CfwPort) {
            Some(_) => match self.camera.get_parameter(Control::CfwPort) {
                //the parameter uses ASCII values to represent the position
                Ok(position) => Ok((position - 48_f64) as u32), //removing ASCII offset
                Err(error) => {
                    tracing::error!(error = ?error);
                    Err(eyre!(error))
                }
            },
            None => {
                tracing::debug!("No filter wheel plugged in.");
                Err(eyre!(GetCfwPositionError))
            }
        }
    }

    /// Sets the current filter wheel position
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,FilterWheel};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let fw = sdk.filter_wheels().last().expect("no filter wheel found");
    /// fw.open().expect("open failed");
    /// fw.set_fw_position(1).expect("set_fw_position failed");
    /// ```
    pub fn set_fw_position(&self, position: u32) -> Result<()> {
        match self.camera.is_control_available(Control::CfwPort) {
            //the parameter uses ASCII values to represent the position
            Some(_) => self
                .camera
                .set_parameter(Control::CfwPort, (position + 48_u32) as f64) //adding ASCII offset
                .map_err(|_| {
                    let error = SetCfwPositionError;
                    tracing::error!(error = ?error);
                    eyre!(error)
                }),
            None => {
                tracing::debug!("No filter wheel plugged in.");
                Err(eyre!(SetCfwPositionError))
            }
        }
    }
}

// Unit tests requiring FFI mocking are in src/tests/
// Simulation integration tests are in tests/simulation/
#[cfg(test)]
mod tests;
