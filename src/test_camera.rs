use super::*;
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDModel_context, OpenQHYCCD_context, SetQHYCCDReadMode_context,
    SetQHYCCDStreamMode_context, QHYCCD_SUCCESS,
};

use crate::QHYError::{SetReadoutModeError, SetStreamModeError};

fn new_camera() -> Camera {
    let ctx_open = OpenQHYCCD_context();
    ctx_open
        .expect()
        .times(1)
        .return_const_st(0xdeadbeef as *const std::ffi::c_void);
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
        SetStreamModeError {
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
        SetReadoutModeError {
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
}
