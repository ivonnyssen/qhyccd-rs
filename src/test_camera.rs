use super::*;
use crate::mocks::mock_libqhyccd_sys::{
    BeginQHYCCDLive_context, CancelQHYCCDExposingAndReadout_context, CancelQHYCCDExposing_context,
    CloseQHYCCD_context, ExpQHYCCDSingleFrame_context, GetQHYCCDChipInfo_context,
    GetQHYCCDEffectiveArea_context, GetQHYCCDExposureRemaining_context, GetQHYCCDFWVersion_context,
    GetQHYCCDLiveFrame_context, GetQHYCCDMemLength_context, GetQHYCCDModel_context,
    GetQHYCCDNumberOfReadModes_context, GetQHYCCDOverScanArea_context, GetQHYCCDParam_context,
    GetQHYCCDReadModeName_context, GetQHYCCDReadModeResolution_context, GetQHYCCDReadMode_context,
    GetQHYCCDSingleFrame_context, GetQHYCCDType_context, InitQHYCCD_context,
    IsQHYCCDControlAvailable_context, OpenQHYCCD_context, SetQHYCCDBinMode_context,
    SetQHYCCDBitsMode_context, SetQHYCCDDebayerOnOff_context, SetQHYCCDParam_context,
    SetQHYCCDReadMode_context, SetQHYCCDResolution_context, SetQHYCCDStreamMode_context,
    StopQHYCCDLive_context, QHYCCD_SUCCESS,
};

const TEST_HANDLE: *const std::ffi::c_void = 0xdeadbeef as *const std::ffi::c_void;

fn new_camera() -> Camera {
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(1).return_const_st(TEST_HANDLE);
    Camera::new("test_camera".to_owned()).unwrap()
}

#[test]
fn new_fail_null_error() {
    //given
    let s = "abc\0def".to_owned();
    //when
    let res = Camera::new(s);
    //then
    assert!(res.is_err());
}

#[test]
fn set_stream_mode_success() {
    //given
    let ctx = SetQHYCCDStreamMode_context();
    ctx.expect()
        .withf_st(|_, mode| *mode == StreamMode::LiveMode as u8)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_stream_mode(StreamMode::LiveMode);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_stream_mode_fail() {
    //given
    let ctx = SetQHYCCDStreamMode_context();
    ctx.expect().times(1).return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_stream_mode(StreamMode::LiveMode);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetStreamModeError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn set_readout_mode_success() {
    //given
    let ctx = SetQHYCCDReadMode_context();
    ctx.expect()
        .withf_st(|_, mode| *mode == 1_u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_readout_mode(1_u32);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_readout_mode_fail() {
    //given
    let ctx = SetQHYCCDReadMode_context();
    ctx.expect().times(1).return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_readout_mode(1_u32);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetReadoutModeError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_model_success() {
    //given
    let ctx = GetQHYCCDModel_context();
    ctx.expect().times(1).returning_st(|_handle, model| unsafe {
        let cam_model = "QHY178M\0";
        model.copy_from(cam_model.as_ptr() as *const c_char, cam_model.len());

        QHYCCD_SUCCESS
    });
    let cam = new_camera();
    //when
    let res = cam.get_model();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "QHY178M");
}

#[test]
fn get_model_fail() {
    //given
    let ctx = GetQHYCCDModel_context();
    ctx.expect().times(1).return_const(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_model();
    //then
    assert!(res.is_err());
}

#[test]
fn get_model_utf8_error() {
    //given
    let ctx = GetQHYCCDModel_context();
    ctx.expect().times(1).returning_st(|_handle, model| unsafe {
        let cam_model = b"\xc3\x28\0";
        model.copy_from(cam_model.as_ptr() as *const c_char, cam_model.len());

        QHYCCD_SUCCESS
    });
    let cam = new_camera();
    //when
    let res = cam.get_model();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "invalid utf-8 sequence of 1 bytes from index 0"
    );
}

#[test]
fn init_success() {
    //given
    let ctx = InitQHYCCD_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.init();
    //then
    assert!(res.is_ok());
}

#[test]
fn init_fail() {
    //given
    let ctx = InitQHYCCD_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.init();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::InitCameraError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_firmware_version_success() {
    //given
    let ctx = GetQHYCCDFWVersion_context();
    ctx.expect()
        .times(1)
        .returning_st(|_handle, version| unsafe {
            let fw_version = b"\x01\x23\0";
            version.copy_from(fw_version.as_ptr(), fw_version.len());

            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_firmware_version();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "Firmware version: 2016_1_35");

    //given
    let ctx = GetQHYCCDFWVersion_context();
    ctx.expect()
        .times(1)
        .returning_st(|_handle, version| unsafe {
            let fw_version = b"\xA1\x11\0";
            version.copy_from(fw_version.as_ptr(), fw_version.len());

            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_firmware_version();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "Firmware version: 2010_1_17");
}

#[test]
fn get_firmware_version_fail() {
    //given
    let ctx = GetQHYCCDFWVersion_context();
    ctx.expect()
        .withf_st(|handle, _version| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_firmware_version();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetFirmwareVersionError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_number_of_readout_modes_success() {
    //given
    let ctx = GetQHYCCDNumberOfReadModes_context();
    ctx.expect()
        .withf_st(|handle, _number| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, number| unsafe {
            *number = 2;
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_number_of_readout_modes();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn get_number_of_readout_modes_fail() {
    //given
    let ctx = GetQHYCCDNumberOfReadModes_context();
    ctx.expect()
        .withf_st(|handle, _number| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_number_of_readout_modes();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetNumberOfReadoutModesError.to_string()
    );
}

#[test]
fn get_readout_mode_name_success() {
    //given
    let ctx = GetQHYCCDReadModeName_context();
    ctx.expect()
        .withf_st(|handle, _index, _mode| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, _index, mode| unsafe {
            let read_mode = "STANDARD MODE\0";
            mode.copy_from(read_mode.as_ptr() as *const c_char, read_mode.len());

            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode_name(0);
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "STANDARD MODE");
}

#[test]
fn get_readout_mode_name_fail() {
    //given
    let ctx = GetQHYCCDReadModeName_context();
    ctx.expect()
        .withf_st(|handle, _index, _mode| *handle == TEST_HANDLE)
        .times(1)
        .return_const(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode_name(0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetReadoutModeNameError.to_string()
    );
}

#[test]
fn get_readout_mode_name_utf8_error() {
    //given
    let ctx = GetQHYCCDReadModeName_context();
    ctx.expect()
        .withf_st(|handle, _index, _mode| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, _index, mode| unsafe {
            let read_mode = b"\xc3\x28\0";
            mode.copy_from(read_mode.as_ptr() as *const c_char, read_mode.len());

            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode_name(0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "invalid utf-8 sequence of 1 bytes from index 0"
    );
}

#[test]
fn get_readout_mode_resolution_success() {
    //given
    let ctx = GetQHYCCDReadModeResolution_context();
    ctx.expect()
        .withf_st(|handle, _index, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, _index, width, height| unsafe {
            *width = 1024;
            *height = 768;

            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode_resolution(0);
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), (1024, 768));
}

#[test]
fn get_readout_mode_resolution_fail() {
    //given
    let ctx = GetQHYCCDReadModeResolution_context();
    ctx.expect()
        .withf_st(|handle, _index, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .return_const(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode_resolution(0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetReadoutModeResolutionError.to_string()
    );
}

#[test]
fn get_readout_mode_success() {
    //given
    let ctx = GetQHYCCDReadMode_context();
    ctx.expect()
        .withf_st(|handle, _mode| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, mode| unsafe {
            *mode = 1;
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn get_readout_mode_fail() {
    //given
    let ctx = GetQHYCCDReadMode_context();
    ctx.expect()
        .withf_st(|handle, _mode| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_readout_mode();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetReadoutModeError.to_string()
    );
}

#[test]
fn get_type_success() {
    //given
    let ctx = GetQHYCCDType_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(42_u32);
    let cam = new_camera();
    //when
    let res = cam.get_type();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 42_u32);
}

#[test]
fn get_type_fail() {
    //given
    let ctx = GetQHYCCDType_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_type();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetCameraTypeError.to_string()
    );
}

#[test]
fn set_bin_mode_success() {
    //given
    let ctx = SetQHYCCDBinMode_context();
    ctx.expect()
        .withf_st(|handle, bin_x, bin_y| {
            *handle == TEST_HANDLE && *bin_x == 2_u32 && *bin_y == 2_u32
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_bin_mode(2, 2);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_bin_mode_fail() {
    //given
    let ctx = SetQHYCCDBinMode_context();
    ctx.expect()
        .withf_st(|handle, bin_x, bin_y| {
            *handle == TEST_HANDLE && *bin_x == 2_u32 && *bin_y == 2_u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_bin_mode(2, 2);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetBinModeError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn set_debayer_success() {
    //given
    let ctx = SetQHYCCDDebayerOnOff_context();
    ctx.expect()
        .withf_st(|handle, on| *handle == TEST_HANDLE && *on)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_debayer(true);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_debayer_fail() {
    //given
    let ctx = SetQHYCCDDebayerOnOff_context();
    ctx.expect()
        .withf_st(|handle, on| *handle == TEST_HANDLE && *on)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_debayer(true);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetDebayerError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn set_roi_success() {
    //given
    let ctx = SetQHYCCDResolution_context();
    ctx.expect()
        .withf_st(|handle, start_x, stary_y, width, height| {
            *handle == TEST_HANDLE
                && *start_x == 0_u32
                && *stary_y == 0_u32
                && *width == 1024_u32
                && *height == 768_u32
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_roi(CCDChipArea {
        start_x: 0,
        start_y: 0,
        width: 1024,
        height: 768,
    });
    //then
    assert!(res.is_ok());
}

#[test]
fn set_roi_fail() {
    //given
    let ctx = SetQHYCCDResolution_context();
    ctx.expect()
        .withf_st(|handle, start_x, stary_y, width, height| {
            *handle == TEST_HANDLE
                && *start_x == 0_u32
                && *stary_y == 0_u32
                && *width == 1024_u32
                && *height == 768_u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_roi(CCDChipArea {
        start_x: 0,
        start_y: 0,
        width: 1024,
        height: 768,
    });
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetRoiError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn begin_live_success() {
    //given
    let ctx = BeginQHYCCDLive_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.begin_live();
    //then
    assert!(res.is_ok());
}

#[test]
fn begin_live_fail() {
    //given
    let ctx = BeginQHYCCDLive_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.begin_live();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::BeginLiveError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn end_live_success() {
    //given
    let ctx = StopQHYCCDLive_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.end_live();
    //then
    assert!(res.is_ok());
}

#[test]
fn end_live_fail() {
    //given
    let ctx = StopQHYCCDLive_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.end_live();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::EndLiveError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_image_size_success() {
    //given
    let ctx = GetQHYCCDMemLength_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(42_u32);
    let cam = new_camera();
    //when
    let res = cam.get_image_size();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 42);
}

#[test]
fn get_image_size_fail() {
    //given
    let ctx = GetQHYCCDMemLength_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_image_size();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetImageSizeError.to_string()
    );
}

#[test]
fn get_live_frame_success() {
    //given
    let ctx = GetQHYCCDLiveFrame_context();
    ctx.expect()
        .withf_st(|handle, _width, _height, _bpp, _channels, _buffer| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, width, height, bpp, channels, buffer| unsafe {
            *width = 2;
            *height = 2;
            *bpp = 8;
            *channels = 1;
            let test_image = b"\x01\x02\x03\x04";
            buffer.copy_from(test_image.as_ptr(), 4);
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_live_frame(4);
    //then
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        ImageData {
            data: vec![0x01, 0x02, 0x03, 0x04],
            width: 2,
            height: 2,
            bits_per_pixel: 8,
            channels: 1
        }
    )
}

#[test]
fn get_live_frame_fail() {
    //given
    let ctx = GetQHYCCDLiveFrame_context();
    ctx.expect()
        .withf_st(|handle, _width, _height, _bpp, _channels, _buffer| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_live_frame(4);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetLiveFrameError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_single_frame_success() {
    //given
    let ctx = GetQHYCCDSingleFrame_context();
    ctx.expect()
        .withf_st(|handle, _width, _height, _bpp, _channels, _buffer| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, width, height, bpp, channels, buffer| unsafe {
            *width = 2;
            *height = 2;
            *bpp = 8;
            *channels = 1;
            let test_image = b"\x01\x02\x03\x04";
            buffer.copy_from(test_image.as_ptr(), 4);
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_single_frame(4);
    //then
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        ImageData {
            data: vec![0x01, 0x02, 0x03, 0x04],
            width: 2,
            height: 2,
            bits_per_pixel: 8,
            channels: 1
        }
    )
}

#[test]
fn get_single_frame_fail() {
    //given
    let ctx = GetQHYCCDSingleFrame_context();
    ctx.expect()
        .withf_st(|handle, _width, _height, _bpp, _channels, _buffer| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_single_frame(4);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetSingleFrameError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_overscan_area_success() {
    //given
    let ctx = GetQHYCCDOverScanArea_context();
    ctx.expect()
        .withf_st(|handle, _start_x, _start_y, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, start_x, start_y, width, height| unsafe {
            *start_x = 2;
            *start_y = 5;
            *width = 1024;
            *height = 768;
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_overscan_area();
    //then
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        CCDChipArea {
            start_x: 2,
            start_y: 5,
            width: 1024,
            height: 768
        }
    )
}

#[test]
fn get_overscan_area_fail() {
    //given
    let ctx = GetQHYCCDOverScanArea_context();
    ctx.expect()
        .withf_st(|handle, _start_x, _start_y, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_overscan_area();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetOverscanAreaError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_effective_area_success() {
    //given
    let ctx = GetQHYCCDEffectiveArea_context();
    ctx.expect()
        .withf_st(|handle, _start_x, _start_y, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .returning_st(|_handle, start_x, start_y, width, height| unsafe {
            *start_x = 0;
            *start_y = 0;
            *width = 1024;
            *height = 768;
            QHYCCD_SUCCESS
        });
    let cam = new_camera();
    //when
    let res = cam.get_effective_area();
    //then
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        CCDChipArea {
            start_x: 0,
            start_y: 0,
            width: 1024,
            height: 768
        }
    )
}

#[test]
fn get_effective_area_fail() {
    //given
    let ctx = GetQHYCCDEffectiveArea_context();
    ctx.expect()
        .withf_st(|handle, _start_x, _start_y, _width, _height| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_effective_area();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetEffectiveAreaError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn start_single_frame_exposure_success() {
    //given
    let ctx = ExpQHYCCDSingleFrame_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.start_single_frame_exposure();
    //then
    assert!(res.is_ok());
}

#[test]
fn start_single_frame_exposure_fail() {
    //given
    let ctx = ExpQHYCCDSingleFrame_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.start_single_frame_exposure();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::StartSingleFrameExposureError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_remaining_exposure_us_success() {
    //given
    let ctx = GetQHYCCDExposureRemaining_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(42_u32);
    let cam = new_camera();
    //when
    let res = cam.get_remaining_exposure_us();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0);
    //given
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(42000_u32);
    //when
    let res = cam.get_remaining_exposure_us();
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 42000);
}

#[test]
fn get_remaining_exposure_us_fail() {
    //given
    let ctx = GetQHYCCDExposureRemaining_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_remaining_exposure_us();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetExposureRemainingError.to_string()
    );
}

#[test]
fn stop_exposure_success() {
    //given
    let ctx = CancelQHYCCDExposing_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.stop_exposure();
    //then
    assert!(res.is_ok());
}

#[test]
fn stop_exposure_fail() {
    //given
    let ctx = CancelQHYCCDExposing_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.stop_exposure();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::StopExposureError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn abort_exposure_and_readout_success() {
    //given
    let ctx = CancelQHYCCDExposingAndReadout_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.abort_exposure_and_readout();
    //then
    assert!(res.is_ok());
}

#[test]
fn abort_exposure_and_readout_fail() {
    //given
    let ctx = CancelQHYCCDExposingAndReadout_context();
    ctx.expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.abort_exposure_and_readout();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::AbortExposureAndReadoutError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn is_control_available_success() {
    //given
    let ctx = IsQHYCCDControlAvailable_context();
    ctx.expect()
        .withf_st(|handle, _control| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.is_control_available(Control::Brightness);
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), QHYCCD_SUCCESS)
}

#[test]
fn is_control_available_fail() {
    //given
    let ctx = IsQHYCCDControlAvailable_context();
    ctx.expect()
        .withf_st(|handle, _control| *handle == TEST_HANDLE)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.is_control_available(Control::Brightness);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::IsFeatureSupportedError {
            feature: Control::Brightness
        }
        .to_string()
    );
}

#[test]
fn get_ccd_info_success() {
    //given
    let ctx = GetQHYCCDChipInfo_context();
    ctx.expect()
        .withf_st(
            |handle, _chipw, _chiph, _imagew, _imageh, _pixelw, _pixelh, _bpp| {
                *handle == TEST_HANDLE
            },
        )
        .times(1)
        .returning_st(
            |_handle, chipw, chiph, imagew, imageh, pixelw, pixelh, bpp| unsafe {
                *chipw = 3124.1;
                *chiph = 500.5;
                *imagew = 1024;
                *imageh = 768;
                *pixelw = 2.4;
                *pixelh = 2.4;
                *bpp = 16;
                QHYCCD_SUCCESS
            },
        );
    let cam = new_camera();
    //when
    let res = cam.get_ccd_info();
    //then
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        CCDChipInfo {
            chip_width: 3124.1,
            chip_height: 500.5,
            image_width: 1024,
            image_height: 768,
            pixel_width: 2.4,
            pixel_height: 2.4,
            bits_per_pixel: 16
        }
    )
}

#[test]
fn get_ccd_info_fail() {
    //given
    let ctx = GetQHYCCDChipInfo_context();
    ctx.expect()
        .withf_st(
            |handle, _chipw, _chiph, _imagew, _imageh, _pixelw, _pixelh, _bpp| {
                *handle == TEST_HANDLE
            },
        )
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.get_ccd_info();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetCCDInfoError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn set_bit_mode_success() {
    //given
    let ctx = SetQHYCCDBitsMode_context();
    ctx.expect()
        .withf_st(|handle, mode| *handle == TEST_HANDLE && *mode == 0_u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_bit_mode(0);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_bit_mode_fail() {
    //given
    let ctx = SetQHYCCDBitsMode_context();
    ctx.expect()
        .withf_st(|handle, mode| *handle == TEST_HANDLE && *mode == 0_u32)
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_bit_mode(0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetBitModeError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn get_parameter_success() {
    //given
    let ctx = GetQHYCCDParam_context();
    ctx.expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(5.0);
    let cam = new_camera();
    //when
    let res = cam.get_parameter(Control::CfwSlotsNum);
    //then
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 5.0);
}

#[test]
fn get_parameter_fail() {
    //given
    let ctx = GetQHYCCDParam_context();
    ctx.expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR_F64);
    let cam = new_camera();
    //when
    let res = cam.get_parameter(Control::CfwSlotsNum);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::GetParameterError {
            control: Control::CfwSlotsNum
        }
        .to_string()
    );
}

#[test]
fn set_parameter_success() {
    //given
    let ctx = SetQHYCCDParam_context();
    ctx.expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32 && *value == 16.0
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_parameter(Control::TransferBit, 16.0);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_parameter_fail() {
    //given
    let ctx = SetQHYCCDParam_context();
    ctx.expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32 && *value == 16.0
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_parameter(Control::TransferBit, 16.0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetParameterError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn set_if_available_success() {
    //given
    let ctx_get = IsQHYCCDControlAvailable_context();
    ctx_get
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);

    let ctx_set = SetQHYCCDParam_context();
    ctx_set
        .expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32 && *value == 16.0
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.set_if_available(Control::TransferBit, 16.0);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_if_available_fail() {
    //given
    let ctx_get = IsQHYCCDControlAvailable_context();
    ctx_get
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);

    /*     let ctx_set = SetQHYCCDParam_context();
       ctx_set
           .expect()
           .withf_st(|handle, control, value| {
               *handle == TEST_HANDLE && *control == Control::TransferBit as u32 && *value == 16.0
           })
           .times(1)
           .return_const_st(QHYCCD_ERROR);
    */
    let cam = new_camera();
    //when
    let res = cam.set_if_available(Control::TransferBit, 16.0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::IsFeatureSupportedError {
            feature: Control::TransferBit
        }
        .to_string()
    );

    //given
    let ctx_get = IsQHYCCDControlAvailable_context();
    ctx_get
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);

    let ctx_set = SetQHYCCDParam_context();
    ctx_set
        .expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::TransferBit as u32 && *value == 16.0
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.set_if_available(Control::TransferBit, 16.0);
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::SetParameterError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn open_success() {
    //given
    let mut cam = new_camera();
    let ctx = OpenQHYCCD_context();
    ctx.expect().times(1).return_const_st(TEST_HANDLE);
    //when
    let res = cam.open();
    //then
    assert!(res.is_ok());
}

#[test]
fn open_fail() {
    //given
    let mut cam = new_camera();
    let ctx = OpenQHYCCD_context();
    ctx.expect().times(1).return_const_st(core::ptr::null());
    //when
    let res = cam.open();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::OpenCameraError.to_string()
    );
}

#[test]
fn close_success() {
    //given
    let ctx = CloseQHYCCD_context();
    ctx.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let cam = new_camera();
    //when
    let res = cam.close();
    //then
    assert!(res.is_ok());
}

#[test]
fn close_fail() {
    //given
    let ctx = CloseQHYCCD_context();
    ctx.expect().times(1).return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.close();
    //then
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        QHYError::CloseCameraError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn filter_wheel() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let ctx_num = GetQHYCCDParam_context();
    ctx_num
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(7.0);

    let cam = new_camera();
    //when
    let res = cam.positions();
    //then
    assert_eq!(res, 7);

    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);

    let cam = new_camera();
    //when
    let res = cam.positions();
    //then
    assert_eq!(res, 0);

    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .times(1)
        .return_const_st(QHYCCD_ERROR);
    let cam = new_camera();
    //when
    let res = cam.positions();
    //then
    assert_eq!(res, 0);
}
