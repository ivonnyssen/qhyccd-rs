use std::ffi::c_char;
use std::ffi::CStr;

use eyre::eyre;
use eyre::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QHYError {
    #[error("Error initializing QHYCCD SDK, error code {}", error_code)]
    InitSDKError { error_code: u32 },
    #[error("Error closing QHYCCD SDK, error code {}", error_code)]
    CloseSDKError { error_code: u32 },
    #[error("Error getting QHYCCD SDK version, error code {:?}", error_code)]
    GetSDKVersionError { error_code: u32 },
    #[error("Error scanning QHYCCD cameras")]
    ScanQHYCCDError,
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
    #[error("Error getting determining support for camera feature {:?}", feature)]
    IsFeatureSupportedError { feature: CameraFeature },
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
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct QhyccdHandle {
    ptr: libqhyccd_sys::QhyccdHandle,
}

unsafe impl Send for QhyccdHandle {}
unsafe impl Sync for QhyccdHandle {}

impl QhyccdHandle {
    pub fn new(ptr: libqhyccd_sys::QhyccdHandle) -> Self {
        Self { ptr }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CameraFeature {
    ControlBrightness = 0,
    ControlContrast = 1,
    ControlWbr = 2,
    ControlWbb = 3,
    ControlWbg = 4,
    ControlGamma = 5,
    ControlGain = 6,
    ControlOffset = 7,
    ControlExposure = 8,
    ControlSpeed = 9,
    ControlTransferBit = 10,
    ControlChannels = 11,
    ControlUsbTraffic = 12,
    ControlRowDeNoise = 13,
    ControlCurTemp = 14,
    ControlCurPWM = 15,
    ControlManulPWM = 16,
    ControlCfwPort = 17,
    ControlCooler = 18,
    ControlSt4Port = 19,
    CamColor = 20,
    CamBin1x1mode = 21,
    CamBin2x2mode = 22,
    CamBin3x3mode = 23,
    CamBin4x4mode = 24,
    CamMechanicalShutter = 25,
    CamTrigerInterface = 26,
    CamTecoverprotectInterface = 27,
    CamSignalClampInterface = 28,
    CamFinetoneInterface = 29,
    CamShutterMotorHeatingInterface = 30,
    CamCalibrateFpnInterface = 31,
    CamChipTemperatureSensorInterface = 32,
    CamUsbReadoutSlowestInterface = 33,
    Cam8bits = 34,
    Cam16bits = 35,
    CamGps = 36,
    CamIgnoreOverscanInterface = 37,
    Qhyccd3aAutoexposure = 39,
    Qhyccd3aAutofocus = 40,
    ControlAmpv = 41,
    ControlVcam = 42,
    CamViewMode = 43,
    ControlCfwSlotsNum = 44,
    IsExposingDone = 45,
    ScreenStretchB = 46,
    ScreenStretchW = 47,
    ControlDDR = 48,
    CamLightPerformanceMode = 49,
    CamQhy5iiGuideMode = 50,
    DDRBufferCapacity = 51,
    DDRBufferReadThreshold = 52,
    DefaultGain = 53,
    DefaultOffset = 54,
    OutputDataActualBits = 55,
    OutputDataAlignment = 56,
    CamSingleFrameMode = 57,
    CamLiveVideoMode = 58,
    CamIsColor = 59,
    HasHardwareFrameCounter = 60,
    ControlMaxIdError = 61,
    CamHumidity = 62,
    CamPressure = 63,
    ControlVacuumPump = 64,
    ControlSensorChamberCyclePump = 65,
    Cam32bits = 66,
    CamSensorUlvoStatus = 67,
    CamSensorPhaseReTrain = 68,
    CamInitConfigFromFlash = 69,
    CamTriggerMode = 70,
    CamTriggerOut = 71,
    CamBurstMode = 72,
    CamSpeakerLedAlarm = 73,
    CamWatchDogFpga = 74,
    CamBin6x6mode = 75,
    CamBin8x8mode = 76,
    CamGlobalSensorGpsLED = 77,
    ControlImgProc = 78,
    ControlRemoveRbi = 79,
    ControlGlobalReset = 80,
    ControlFrameDetect = 81,
    CamGainDbConversion = 82,
    CamCurveSystemGain = 83,
    CamCurveFullWell = 84,
    CamCurveReadoutNoise = 85,
    ControlMaxId = 86,
    ControlAutowhitebalance = 1024,
    ControlAutoexposure = 1025,
    ControlAutoexpMessureValue = 1026,
    ControlAutoexpMessureMethod = 1027,
    ControlImageStabilization = 1028,
    ControlGaindB = 1029,
}

#[derive(Debug, PartialEq)]
pub enum CameraStreamMode {
    SingleFrameMode = 0,
    LiveMode = 1,
}

#[derive(Debug, PartialEq)]
pub struct CCDChipInfo {
    pub chip_width: f64,
    pub chip_height: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_width: f64,
    pub pixel_height: f64,
    pub bits_per_pixel: u32,
}

#[derive(Debug, PartialEq)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub channels: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CCDChipArea {
    pub start_x: u32,
    pub start_y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq)]
pub enum BayerId {
    BayerGb = 1,
    BayerGr = 2,
    BayerBg = 3,
    BayerRg = 4,
}

#[derive(Debug, PartialEq)]
pub struct ReadoutMode {
    pub id: u32,
    pub name: String,
}

/// initialize the QHYCCD SDK
///
/// # Example
///
/// ```no_run
/// # use qhyccd_rs::init_sdk;
/// use eyre::Result;
///
/// let result = init_sdk();
/// assert!(result.is_ok());
/// ```
pub fn init_sdk() -> Result<()> {
    match unsafe { libqhyccd_sys::InitQHYCCDResource() } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::InitSDKError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn release_sdk() -> Result<()> {
    match unsafe { libqhyccd_sys::ReleaseQHYCCDResource() } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::CloseSDKError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}
/// Scan the connected cameras and return the nubmer found
///
/// # Example
///
/// ```no_run
/// # use qhyccd_rs::scan_qhyccd;
/// use eyre::Result;
///
/// let result = scan_qhyccd().unwrap();
/// assert_eq!(result, 0);
/// ```
pub fn scan_qhyccd() -> Result<u32> {
    match unsafe { libqhyccd_sys::ScanQHYCCD() } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::ScanQHYCCDError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        num => Ok(num),
    }
}

#[derive(Debug, PartialEq)]
pub struct SDKVersion {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub subday: u32,
}

/// Get the QHYCCD SDK version
/// # Example
/// ```no_run
/// # use qhyccd_rs::get_sdk_version;
/// # use qhyccd_rs::SDKVersion;
/// use eyre::Result;
///
/// let version = get_sdk_version().unwrap();
/// assert_eq!(version, SDKVersion{year: 23, month: 9, day: 6, subday: 14});
/// ```
/// # Errors
/// Returns `QHYError::GetSDKVersionError` if the SDK version cannot be retrieved
pub fn get_sdk_version() -> Result<SDKVersion> {
    let mut year: u32 = 0;
    let mut month: u32 = 0;
    let mut day: u32 = 0;
    let mut subday: u32 = 0;
    match unsafe {
        libqhyccd_sys::GetQHYCCDSDKVersion(&mut year, &mut month, &mut day, &mut subday)
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(SDKVersion {
            year,
            month,
            day,
            subday,
        }),
        error_code => {
            let error = QHYError::GetSDKVersionError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_camera_id(index: u32) -> Result<String> {
    let mut id: [c_char; 32] = [0; 32];
    unsafe {
        match libqhyccd_sys::GetQHYCCDId(index, id.as_mut_ptr()) {
            libqhyccd_sys::QHYCCD_SUCCESS => {
                let id = match CStr::from_ptr(id.as_ptr()).to_str() {
                    Ok(id) => id,
                    Err(error) => {
                        tracing::error!(error = error.to_string().as_str());
                        return Err(eyre!(error));
                    }
                };
                Ok(id.to_string())
            }
            error_code => {
                let error = QHYError::GetCameraIdError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }
}

pub fn open_camera(id: String) -> Result<QhyccdHandle> {
    unsafe {
        match std::ffi::CString::new(id) {
            Ok(id) => {
                let handle = libqhyccd_sys::OpenQHYCCD(id.as_ptr());
                Ok(QhyccdHandle::new(handle))
            }
            Err(error) => {
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }
}

pub fn close_camera(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::CloseQHYCCD(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::CloseCameraError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_firmware_version(handle: QhyccdHandle) -> Result<String> {
    let mut version = [0u8; 32];
    match unsafe { libqhyccd_sys::GetQHYCCDFWVersion(handle.ptr, version.as_mut_ptr()) } {
        libqhyccd_sys::QHYCCD_SUCCESS => {
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
            let error = QHYError::GetFirmwareVersionError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn is_feature_supported(handle: QhyccdHandle, feature: CameraFeature) -> Result<u32> {
    match unsafe { libqhyccd_sys::IsQHYCCDControlAvailable(handle.ptr, feature as u32) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::IsFeatureSupportedError { feature };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        is_supported => Ok(is_supported),
    }
}

pub fn set_readout_mode(handle: QhyccdHandle, mode: u32) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDReadMode(handle.ptr, mode) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetReadoutModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_stream_mode(handle: QhyccdHandle, mode: CameraStreamMode) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDStreamMode(handle.ptr, mode as u8) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetStreamModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn init_camera(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::InitQHYCCD(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::InitCameraError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_ccd_info(handle: QhyccdHandle) -> Result<CCDChipInfo> {
    let mut chipw: f64 = 0.0;
    let mut chiph: f64 = 0.0;
    let mut imagew: u32 = 0;
    let mut imageh: u32 = 0;
    let mut pixelw: f64 = 0.0;
    let mut pixelh: f64 = 0.0;
    let mut bpp: u32 = 0;
    match unsafe {
        libqhyccd_sys::GetQHYCCDChipInfo(
            handle.ptr,
            &mut chipw as *mut f64,
            &mut chiph as *mut f64,
            &mut imagew as *mut u32,
            &mut imageh as *mut u32,
            &mut pixelw as *mut f64,
            &mut pixelh as *mut f64,
            &mut bpp as *mut u32,
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(CCDChipInfo {
            chip_width: chipw,
            chip_height: chiph,
            image_width: imagew,
            image_height: imageh,
            pixel_width: pixelw,
            pixel_height: pixelh,
            bits_per_pixel: bpp,
        }),
        error_code => {
            let error = QHYError::GetCCDInfoError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_bit_mode(handle: QhyccdHandle, mode: u32) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDBitsMode(handle.ptr, mode) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetBitModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_debayer(handle: QhyccdHandle, on: bool) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDDebayerOnOff(handle.ptr, on) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetDebayerError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_bin_mode(handle: QhyccdHandle, bin_x: u32, bin_y: u32) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDBinMode(handle.ptr, bin_x, bin_y) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetBinModeError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_roi(handle: QhyccdHandle, roi: CCDChipArea) -> Result<()> {
    match unsafe {
        libqhyccd_sys::SetQHYCCDResolution(
            handle.ptr,
            roi.start_x,
            roi.start_y,
            roi.width,
            roi.height,
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetRoiError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn set_parameter(handle: QhyccdHandle, feature: CameraFeature, value: f64) -> Result<()> {
    match unsafe { libqhyccd_sys::SetQHYCCDParam(handle.ptr, feature as u32, value) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::SetParameterError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn begin_live(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::BeginQHYCCDLive(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::BeginLiveError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn end_live(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::StopQHYCCDLive(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::EndLiveError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_image_size(handle: QhyccdHandle) -> Result<usize> {
    match unsafe { libqhyccd_sys::GetQHYCCDMemLength(handle.ptr) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::GetImageSizeError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        size => Ok(size as usize),
    }
}

pub fn get_live_frame(handle: QhyccdHandle, buffer_size: usize) -> Result<ImageData> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bpp: u32 = 0;
    let mut channels: u32 = 0;
    let mut buffer = vec![0u8; buffer_size];
    match unsafe {
        libqhyccd_sys::GetQHYCCDLiveFrame(
            handle.ptr,
            &mut width as *mut u32,
            &mut height as *mut u32,
            &mut bpp as *mut u32,
            &mut channels as *mut u32,
            buffer.as_mut_ptr(),
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(ImageData {
            data: buffer,
            width,
            height,
            bits_per_pixel: bpp,
            channels,
        }),
        error_code => {
            let error = QHYError::GetLiveFrameError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_single_frame(handle: QhyccdHandle, buffer_size: usize) -> Result<ImageData> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bpp: u32 = 0;
    let mut channels: u32 = 0;
    let mut buffer = vec![0u8; buffer_size];
    match unsafe {
        libqhyccd_sys::GetQHYCCDSingleFrame(
            handle.ptr,
            &mut width as *mut u32,
            &mut height as *mut u32,
            &mut bpp as *mut u32,
            &mut channels as *mut u32,
            buffer.as_mut_ptr(),
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(ImageData {
            data: buffer,
            width,
            height,
            bits_per_pixel: bpp,
            channels,
        }),
        error_code => {
            let error = QHYError::GetSingleFrameError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}
pub fn get_overscan_area(handle: QhyccdHandle) -> Result<CCDChipArea> {
    let mut start_x: u32 = 0;
    let mut start_y: u32 = 0;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    match unsafe {
        libqhyccd_sys::GetQHYCCDOverScanArea(
            handle.ptr,
            &mut start_x as *mut u32,
            &mut start_y as *mut u32,
            &mut width as *mut u32,
            &mut height as *mut u32,
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(CCDChipArea {
            start_x,
            start_y,
            width,
            height,
        }),
        error_code => {
            let error = QHYError::GetOverscanAreaError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_effective_area(handle: QhyccdHandle) -> Result<CCDChipArea> {
    let mut start_x: u32 = 0;
    let mut start_y: u32 = 0;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    match unsafe {
        libqhyccd_sys::GetQHYCCDEffectiveArea(
            handle.ptr,
            &mut start_x as *mut u32,
            &mut start_y as *mut u32,
            &mut width as *mut u32,
            &mut height as *mut u32,
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(CCDChipArea {
            start_x,
            start_y,
            width,
            height,
        }),
        error_code => {
            let error = QHYError::GetEffectiveAreaError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn start_single_frame_exposure(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::ExpQHYCCDSingleFrame(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::StartSingleFrameExposureError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_number_of_readout_modes(handle: QhyccdHandle) -> Result<u32> {
    let mut num: u32 = 0;
    match unsafe { libqhyccd_sys::GetQHYCCDNumberOfReadModes(handle.ptr, &mut num as *mut u32) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::GetNumberOfReadoutModesError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        _ => Ok(num),
    }
}

pub fn get_readout_mode_name(handle: QhyccdHandle, index: u32) -> Result<String> {
    let mut name: [c_char; 80] = [0; 80];
    match unsafe { libqhyccd_sys::GetQHYCCDReadModeName(handle.ptr, index, name.as_mut_ptr()) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::GetReadoutModeNameError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        _ => {
            let name = match unsafe { CStr::from_ptr(name.as_ptr()) }.to_str() {
                Ok(name) => name,
                Err(error) => {
                    tracing::error!(error = error.to_string().as_str());
                    return Err(eyre!(error));
                }
            };
            Ok(name.to_string())
        }
    }
}

pub fn get_readout_mode_resolution(handle: QhyccdHandle, index: u32) -> Result<(u32, u32)> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    match unsafe {
        libqhyccd_sys::GetQHYCCDReadModeResolution(
            handle.ptr,
            index,
            &mut width as *mut u32,
            &mut height as *mut u32,
        )
    } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok((width, height)),
        _ => {
            let error = QHYError::GetReadoutModeResolutionError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_readout_mode(handle: QhyccdHandle) -> Result<u32> {
    let mut mode: u32 = 0;
    match unsafe { libqhyccd_sys::GetQHYCCDReadMode(handle.ptr, &mut mode as *mut u32) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(mode),
        _ => {
            let error = QHYError::GetReadoutModeError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_model(handle: QhyccdHandle) -> Result<String> {
    let mut model: [c_char; 80] = [0; 80];
    match unsafe { libqhyccd_sys::GetQHYCCDModel(handle.ptr, model.as_mut_ptr()) } {
        libqhyccd_sys::QHYCCD_SUCCESS => {
            let model = match unsafe { CStr::from_ptr(model.as_ptr()) }.to_str() {
                Ok(model) => model,
                Err(error) => {
                    tracing::error!(error = error.to_string().as_str());
                    return Err(eyre!(error));
                }
            };
            Ok(model.to_string())
        }
        error_code => {
            let error = QHYError::GetCameraModelError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn get_type(handle: QhyccdHandle) -> Result<u32> {
    match unsafe { libqhyccd_sys::GetQHYCCDType(handle.ptr) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::GetCameraTypeError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        camera_type => Ok(camera_type),
    }
}

pub fn get_remaining_exposure_us(handle: QhyccdHandle) -> Result<u32> {
    match unsafe { libqhyccd_sys::GetQHYCCDExposureRemaining(handle.ptr) } {
        libqhyccd_sys::QHYCCD_ERROR => {
            let error = QHYError::GetExposureRemainingError;
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
        remaining if { remaining <= 100 } => Ok(0),
        remaining => Ok(remaining),
    }
}

pub fn stop_exposure(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::CancelQHYCCDExposing(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::StopExposureError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}

pub fn abort_exposure_and_readout(handle: QhyccdHandle) -> Result<()> {
    match unsafe { libqhyccd_sys::CancelQHYCCDExposingAndReadout(handle.ptr) } {
        libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
        error_code => {
            let error = QHYError::AbortExposureAndReadoutError { error_code };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        }
    }
}
