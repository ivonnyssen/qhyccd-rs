use thiserror::Error;

use crate::Control;

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
