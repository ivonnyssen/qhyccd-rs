//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use core::ffi::c_char;
use core::ffi::CStr;

use eyre::eyre;
use eyre::Result;
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
    #[error("Error opening camera, error code")]
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
    #[error("Error getting determining support for camera feature {:?}", feature)]
    IsFeatureSupportedError { feature: Control },
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
/// this struct is returned ffrom `is_control_available` when used with `Control::CamColor`
pub enum BayerId {
    ///GBRG
    BayerGb = 1,
    ///GRBG
    BayerGr = 2,
    ///BGGR
    BayerBg = 3,
    ///RGGB
    BayerRg = 4,
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
/// The representation of the SDK. It automatically allocates teh SDK when constructed
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
}

impl Sdk {
    /// Creates a new instance of the SDK
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new();
    /// assert!(sdk.is_ok());
    /// ```
    pub fn new() -> Result<Self> {
        match unsafe { libqhyccd_sys::InitQHYCCDResource() } {
            libqhyccd_sys::QHYCCD_SUCCESS => {
                let num_cameras = match unsafe { libqhyccd_sys::ScanQHYCCD() } {
                    libqhyccd_sys::QHYCCD_ERROR => {
                        let error = QHYError::ScanQHYCCDError;
                        tracing::error!(error = error.to_string().as_str());
                        Err(eyre!(error))
                    }
                    num => Ok(num),
                }?;

                let mut cameras = Vec::with_capacity(num_cameras as usize);
                for index in 0..num_cameras {
                    let id = {
                        let mut c_id: [c_char; 32] = [0; 32];
                        unsafe {
                            match libqhyccd_sys::GetQHYCCDId(index, c_id.as_mut_ptr()) {
                                libqhyccd_sys::QHYCCD_SUCCESS => {
                                    let id = match CStr::from_ptr(c_id.as_ptr()).to_str() {
                                        Ok(id) => id,
                                        Err(error) => {
                                            tracing::error!(error = error.to_string().as_str());
                                            return Err(eyre!(error));
                                        }
                                    };
                                    Ok(id.to_owned())
                                }
                                error_code => {
                                    let error = QHYError::GetCameraIdError { error_code };
                                    tracing::error!(error = error.to_string().as_str());
                                    Err(eyre!(error))
                                }
                            }
                        }
                    }?;
                    match Camera::new(id) {
                        Ok(camera) => cameras.push(camera),
                        Err(error) => tracing::error!(error = error.to_string().as_str()),
                    }
                }

                Ok(Sdk { cameras })
            }
            error_code => {
                let error = QHYError::InitSDKError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
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
    pub fn filter_wheels(&self) -> impl Iterator<Item = &impl FilterWheel> {
        self.cameras
            .iter()
            .filter(|camera| match camera.is_cfw_plugged_in() {
                Ok(true) => true,
                Ok(false) => false,
                Err(error) => {
                    tracing::error!(error = error.to_string().as_str());
                    false
                }
            })
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
}

impl Drop for Sdk {
    fn drop(&mut self) {
        match unsafe { libqhyccd_sys::ReleaseQHYCCDResource() } {
            libqhyccd_sys::QHYCCD_SUCCESS => (),
            error_code => {
                let error = QHYError::CloseSDKError { error_code };
                tracing::error!(error = error.to_string().as_str());
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The representation of a camera. It is constructed by the SDK and can be used to
/// interact with the camera.
pub struct Camera {
    id: String,
    handle: *const std::ffi::c_void,
}

impl Camera {
    /// Creates a new instance of the camera, the sdk does this automatically so do not use this function directly
    /// but rather use the iterators provided by the sdk to obtain the connected cameras and filter wheels
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// println!("Camera: {:?}", camera);
    /// ```
    pub fn new(id: String) -> Result<Self> {
        unsafe {
            match std::ffi::CString::new(id.clone()) {
                Ok(c_id) => {
                    let handle = libqhyccd_sys::OpenQHYCCD(c_id.as_ptr());
                    if handle.is_null() {
                        let error = QHYError::OpenCameraError;
                        tracing::error!(error = error.to_string().as_str());
                        return Err(eyre!(error));
                    }
                    Ok(Camera { id, handle })
                }
                Err(error) => {
                    tracing::error!(error = error.to_string().as_str());
                    Err(eyre!(error))
                }
            }
        }
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
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// ```
    pub fn set_stream_mode(&self, mode: StreamMode) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDStreamMode(self.handle, mode as u8) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetStreamModeError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    ///
    /// camera.set_readout_mode(0).expect("set_readout_mode failed");
    pub fn set_readout_mode(&self, mode: u32) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDReadMode(self.handle, mode) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetReadoutModeError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }

    /// retunrs the model or short description of the camera - this does not work for all cameras
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let model = camera.get_model().expect("get_model failed");
    /// println!("Camera model: {}", model);
    /// ```
    pub fn get_model(&self) -> Result<String> {
        let mut model: [c_char; 80] = [0; 80];
        match unsafe { libqhyccd_sys::GetQHYCCDModel(self.handle, model.as_mut_ptr()) } {
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

    /// initializes the camera to a new session - use this to change from LiveMode to SingleFrameMode for instance
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk, StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// ```
    pub fn init(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::InitQHYCCD(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::InitCameraError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }

    /// retunrs the firmware version of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let firmware_version = camera.get_firmware_version().expect("get_firmware_version failed");
    /// println!("Firmware version: {}", firmware_version);
    /// ```
    pub fn get_firmware_version(&self) -> Result<String> {
        let mut version = [0u8; 32];
        match unsafe { libqhyccd_sys::GetQHYCCDFWVersion(self.handle, version.as_mut_ptr()) } {
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

    /// Returns the number of readout modes of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// println!("Number of readout modes: {}", num_readout_modes);
    /// ```
    pub fn get_number_of_readout_modes(&self) -> Result<u32> {
        let mut num: u32 = 0;
        match unsafe {
            libqhyccd_sys::GetQHYCCDNumberOfReadModes(self.handle, &mut num as *mut u32)
        } {
            libqhyccd_sys::QHYCCD_ERROR => {
                let error = QHYError::GetNumberOfReadoutModesError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
            _ => Ok(num),
        }
    }

    /// Returns the readout mode name with the given index. Make sure to check the number of readout modes.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// for index in 0..num_readout_modes {
    ///    let readout_mode_name = camera.get_readout_mode_name(index).expect("get_readout_mode_name failed");
    ///   println!("Readout mode {}: {}", index, readout_mode_name);
    /// }
    /// ```
    pub fn get_readout_mode_name(&self, index: u32) -> Result<String> {
        let mut name: [c_char; 80] = [0; 80];
        match unsafe { libqhyccd_sys::GetQHYCCDReadModeName(self.handle, index, name.as_mut_ptr()) }
        {
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

    /// Returns the resolution of the readout mode with the given index. Make sure to check the number of readout modes.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let num_readout_modes = camera.get_number_of_readout_modes().expect("get_number_of_readout_modes failed");
    /// for index in 0..num_readout_modes {
    ///   let readout_mode_resolution = camera.get_readout_mode_resolution(index).expect("get_readout_mode_resolution failed");
    ///  println!("Readout mode {}: {:?}", index, readout_mode_resolution);
    /// }
    /// ```
    pub fn get_readout_mode_resolution(&self, index: u32) -> Result<(u32, u32)> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        match unsafe {
            libqhyccd_sys::GetQHYCCDReadModeResolution(
                self.handle,
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

    /// Returns the current readout mode of the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::Sdk;
    /// use qhyccd_rs::Camera;
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let readout_mode = camera.get_readout_mode().expect("get_readout_mode failed");
    /// println!("Readout mode: {}", readout_mode);
    /// ```
    pub fn get_readout_mode(&self) -> Result<u32> {
        let mut mode: u32 = 0;
        match unsafe { libqhyccd_sys::GetQHYCCDReadMode(self.handle, &mut mode as *mut u32) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(mode),
            _ => {
                let error = QHYError::GetReadoutModeError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// let tipe = camera.get_type().expect("get_type failed");
    /// println!("Type: {}", tipe);
    /// ```
    pub fn get_type(&self) -> Result<u32> {
        match unsafe { libqhyccd_sys::GetQHYCCDType(self.handle) } {
            libqhyccd_sys::QHYCCD_ERROR => {
                let error = QHYError::GetCameraTypeError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
            camera_type => Ok(camera_type),
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
    /// camera.set_bin_mode(2, 2).expect("set_bin_mode failed");
    /// ```
    pub fn set_bin_mode(&self, bin_x: u32, bin_y: u32) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDBinMode(self.handle, bin_x, bin_y) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetBinModeError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// camera.set_debayer(false).expect("set_debayer failed");
    ///```
    pub fn set_debayer(&self, on: bool) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDDebayerOnOff(self.handle, on) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetDebayerError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// let roi = CCDChipArea {
    ///     start_x: 0,
    ///     start_y: 0,
    ///     width: 1000,
    ///     height: 1000,
    /// };
    /// camera.set_roi(roi).expect("set_roi failed");
    /// ```
    pub fn set_roi(&self, roi: CCDChipArea) -> Result<()> {
        match unsafe {
            libqhyccd_sys::SetQHYCCDResolution(
                self.handle,
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

    /// Starts Live Video Mode on the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.begin_live().expect("begin_live failed");
    /// ```
    pub fn begin_live(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::BeginQHYCCDLive(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::BeginLiveError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }

    /// Stops Live Video Mode on the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_stream_mode(StreamMode::LiveMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.begin_live().expect("begin_live failed");
    /// /* Download images in between */
    /// camera.end_live().expect("end_live failed");
    /// ```
    pub fn end_live(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::StopQHYCCDLive(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::EndLiveError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// // wait for exposure to finish
    /// let buffer_size = camera.get_image_size().expect("get_camera_image_size failed");
    /// let image = camera.get_single_frame(buffer_size).expect("get_camera_single_frame failed");
    /// ```
    pub fn get_image_size(&self) -> Result<usize> {
        match unsafe { libqhyccd_sys::GetQHYCCDMemLength(self.handle) } {
            libqhyccd_sys::QHYCCD_ERROR => {
                let error = QHYError::GetImageSizeError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
            size => Ok(size as usize),
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
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut bpp: u32 = 0;
        let mut channels: u32 = 0;
        let mut buffer = vec![0u8; buffer_size];
        match unsafe {
            libqhyccd_sys::GetQHYCCDLiveFrame(
                self.handle,
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

    /// Returns the image stored in the camera as `ImageData` struct if the camera is in Single Frame Mode
    /// # Example
    ///
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 10000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_camera_single_frame_exposure failed");
    /// let buffer_size = camera.get_image_size().expect("get_camera_image_size failed");
    /// let image = camera.get_single_frame(buffer_size).expect("get_camera_single_frame failed");
    /// ```
    pub fn get_single_frame(&self, buffer_size: usize) -> Result<ImageData> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut bpp: u32 = 0;
        let mut channels: u32 = 0;
        let mut buffer = vec![0u8; buffer_size];
        match unsafe {
            libqhyccd_sys::GetQHYCCDSingleFrame(
                self.handle,
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

    /// Get the chip area including overscan area
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,CCDChipArea};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let chip_area = camera.get_overscan_area().expect("get_overscan_area failed");
    /// println!("Chip area: {:?}", chip_area);
    /// ```
    pub fn get_overscan_area(&self) -> Result<CCDChipArea> {
        let mut start_x: u32 = 0;
        let mut start_y: u32 = 0;
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        match unsafe {
            libqhyccd_sys::GetQHYCCDOverScanArea(
                self.handle,
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

    /// Get the effective imaging chip area
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,CCDChipArea};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let chip_area = camera.get_effective_area().expect("get_overscan_area failed");
    /// println!("Chip area: {:?}", chip_area);
    /// ```
    pub fn get_effective_area(&self) -> Result<CCDChipArea> {
        let mut start_x: u32 = 0;
        let mut start_y: u32 = 0;
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        match unsafe {
            libqhyccd_sys::GetQHYCCDEffectiveArea(
                self.handle,
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

    /// Start a long exposure
    /// Make sure to set the exposure time before calling this function
    /// this function blocks the current thread and only returns when the exposure is finished
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,StreamMode,Control, ImageData};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_stream_mode(StreamMode::SingleFrameMode).expect("set_stream_mode failed");
    /// camera.init().expect("init failed");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_param failed"); // this is in micro seconds
    /// camera.start_single_frame_exposure().expect("start_single_frame_exposure failed");
    /// ```
    pub fn start_single_frame_exposure(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::ExpQHYCCDSingleFrame(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::StartSingleFrameExposureError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// /* start exposure on a different thread*/
    /// let remaining_exposure = camera.get_remaining_exposure_us().expect("get_remaining_exposure_us failed");
    /// println!("Remaining exposure: {}", remaining_exposure);
    /// ```
    pub fn get_remaining_exposure_us(&self) -> Result<u32> {
        match unsafe { libqhyccd_sys::GetQHYCCDExposureRemaining(self.handle) } {
            libqhyccd_sys::QHYCCD_ERROR => {
                let error = QHYError::GetExposureRemainingError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
            remaining if { remaining <= 100 } => Ok(0),
            remaining => Ok(remaining),
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
    /// /* start exposure on a different thread*/
    /// camera.stop_exposure().expect("stop_exposure failed");
    /// /* retrieve image data */
    /// ```
    pub fn stop_exposure(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::CancelQHYCCDExposing(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::StopExposureError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// /* start exposure on a different thread*/
    /// camera.abort_exposure_and_readout().expect("abort_exposure failed");
    /// ```
    pub fn abort_exposure_and_readout(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::CancelQHYCCDExposingAndReadout(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::AbortExposureAndReadoutError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// if camera.is_control_available(Control::CamLiveVideoMode).is_err()
    /// {
    ///    println!("Control::CamLiveVideoMode is not supported");
    /// }
    /// let camera_is_color = camera.is_control_available(Control::CamColor).is_ok(); //this returns a `BayerID` if it is a color camera
    /// ```
    pub fn is_control_available(&self, control: Control) -> Result<u32> {
        match unsafe { libqhyccd_sys::IsQHYCCDControlAvailable(self.handle, control as u32) } {
            libqhyccd_sys::QHYCCD_ERROR => {
                let error = QHYError::IsFeatureSupportedError { feature: control };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
            is_supported => Ok(is_supported),
        }
    }

    /// Returns information about the chip in the camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,CCDChipInfo};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let chip_info = camera.get_ccd_info().expect("get_ccd_info failed");
    /// println!("Chip info: {:?}", chip_info);
    /// ```
    pub fn get_ccd_info(&self) -> Result<CCDChipInfo> {
        let mut chipw: f64 = 0.0;
        let mut chiph: f64 = 0.0;
        let mut imagew: u32 = 0;
        let mut imageh: u32 = 0;
        let mut pixelw: f64 = 0.0;
        let mut pixelh: f64 = 0.0;
        let mut bpp: u32 = 0;
        match unsafe {
            libqhyccd_sys::GetQHYCCDChipInfo(
                self.handle,
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

    /// Sets the USB transfer mode to either 8 or 16 bit
    ///
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_bit_mode(8).expect("set_bit_mode failed");
    /// ```
    pub fn set_bit_mode(&self, mode: u32) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDBitsMode(self.handle, mode) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetBitModeError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }

    /// Returns the value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// let number_of_filter_wheel_positions = match camera.is_control_available(Control::CfwSlotsNum).is_ok() {
    ///     true => camera.get_parameter(Control::CfwSlotsNum).unwrap_or_default() as u32,
    ///     false => 0,
    /// };
    /// ```
    pub fn get_parameter(&self, control: Control) -> Result<f64> {
        let res = unsafe { libqhyccd_sys::GetQHYCCDParam(self.handle, control as u32) };
        if (res - libqhyccd_sys::QHYCCD_ERROR_F64).abs() < f64::EPSILON {
            let error = QHYError::GetParameterError { control };
            tracing::error!(error = error.to_string().as_str());
            Err(eyre!(error))
        } else {
            Ok(res)
        }
    }

    /// Sets the value for a given control
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.set_parameter(Control::Exposure, 2000000.0).expect("set_parameter failed");
    /// ```
    pub fn set_parameter(&self, control: Control, value: f64) -> Result<()> {
        match unsafe { libqhyccd_sys::SetQHYCCDParam(self.handle, control as u32, value) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::SetParameterError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
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
    /// camera.set_if_available(Control::TransferBit, 16.0).expect("failed to set usb transfer mode");
    /// ```
    pub fn set_if_available(&self, control: Control, value: f64) -> Result<()> {
        match self.is_control_available(control) {
            Ok(_) => self.set_parameter(control, value),
            Err(e) => Err(e),
        }
    }

    /// Returns `true` if a filter wheel is plugged into the given camera
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera,Control};
    ///
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let filter_wheels = sdk.cameras().filter(|camera| match camera.is_cfw_plugged_in() {
    ///     Ok(true) => true,
    ///     Ok(false) => false,
    ///     Err(error) => false
    /// });
    /// ```
    pub fn is_cfw_plugged_in(&self) -> Result<bool> {
        match unsafe { libqhyccd_sys::IsQHYCCDCFWPlugged(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(true),
            _ => {
                let error = QHYError::IsCfwPluggedInError;
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }

    /// Opens a camera with the given id. The SDK automatically finds all connected cameras upo initialization
    /// so you should never have to call this function.
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let mut camera = Camera::new("QHY178M-222b16468c5966524".to_owned()).expect("Camera::new failed");
    /// camera.open().expect("open failed");
    /// ```
    pub fn open(&mut self) -> Result<()> {
        unsafe {
            match std::ffi::CString::new(self.id.clone()) {
                Ok(c_id) => {
                    let handle = libqhyccd_sys::OpenQHYCCD(c_id.as_ptr());
                    if handle.is_null() {
                        let error = QHYError::OpenCameraError;
                        tracing::error!(error = error.to_string().as_str());
                        return Err(eyre!(error));
                    }
                    self.handle = handle;
                    Ok(())
                }
                Err(error) => {
                    tracing::error!(error = error.to_string().as_str());
                    Err(eyre!(error))
                }
            }
        }
    }

    /// Closes the camera. If you have to call this function, you can then try to open the camera again by
    /// calling `open`
    /// # Example
    /// ```no_run
    /// use qhyccd_rs::{Sdk,Camera};
    /// let sdk = Sdk::new().expect("SDK::new failed");
    /// let camera = sdk.cameras().last().expect("no camera found");
    /// camera.close().expect("close failed");
    /// ```
    pub fn close(&self) -> Result<()> {
        match unsafe { libqhyccd_sys::CloseQHYCCD(self.handle) } {
            libqhyccd_sys::QHYCCD_SUCCESS => Ok(()),
            error_code => {
                let error = QHYError::CloseCameraError { error_code };
                tracing::error!(error = error.to_string().as_str());
                Err(eyre!(error))
            }
        }
    }
}

/// Filter wheels are directly connected to the QHY camera and can be controlled through the camera
pub trait FilterWheel {
    /// Returns the number of filter positions of the filter wheel
    fn positions(&self) -> u32;
}

impl FilterWheel for Camera {
    fn positions(&self) -> u32 {
        match self.is_control_available(Control::CfwSlotsNum).is_ok() {
            true => self.get_parameter(Control::CfwSlotsNum).unwrap_or_default() as u32,
            false => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let sdk = Sdk::new().unwrap();
        assert_eq!(sdk.cameras().count(), 1);
        assert_eq!(sdk.filter_wheels().count(), 1);
        assert_eq!(sdk.filter_wheels().last().unwrap().positions(), 7);

        let camera = sdk.cameras().last().unwrap();
        assert_eq!(
            camera.get_ccd_info().unwrap(),
            CCDChipInfo {
                chip_width: 7334.4,
                chip_height: 4915.2,
                image_width: 3056,
                image_height: 2048,
                pixel_width: 2.4,
                pixel_height: 2.4,
                bits_per_pixel: 16,
            }
        );
        assert_eq!(
            camera.get_firmware_version().unwrap(),
            "Firmware version: 2022_9_5"
        );

        assert_eq!(camera.get_number_of_readout_modes().unwrap(), 1);
        assert_eq!(camera.get_readout_mode_name(0).unwrap(), "STANDARD MODE");
        assert_eq!(camera.get_type().unwrap(), 4010);
        assert!(camera.is_control_available(Control::CamColor).is_err());
    }
}
