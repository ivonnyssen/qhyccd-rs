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
