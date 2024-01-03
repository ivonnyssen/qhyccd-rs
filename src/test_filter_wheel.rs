use super::*;
use crate::mocks::mock_libqhyccd_sys::{
    CloseQHYCCD_context, GetQHYCCDParam_context, IsQHYCCDCFWPlugged_context,
    IsQHYCCDControlAvailable_context, OpenQHYCCD_context, SetQHYCCDParam_context, QHYCCD_SUCCESS,
};

const TEST_HANDLE: *const std::ffi::c_void = 0xdeadbeef as *const std::ffi::c_void;

fn new_filter_wheel() -> FilterWheel {
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(1).return_const_st(TEST_HANDLE);
    let camera = Camera::new("test_camera".to_owned());
    camera.open().unwrap();
    FilterWheel::new(camera)
}

#[test]
fn open_success() {
    //given
    let fw = new_filter_wheel();
    //when
    let res = fw.open();
    //then
    assert!(res.is_ok());
    assert_eq!(fw.id(), "test_camera");
}

#[test]
fn open_fail() {
    //given
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(1).return_const_st(std::ptr::null());
    let camera = Camera::new("test_camera".to_owned());
    let fw = FilterWheel::new(camera);
    //when
    let res = fw.open();
    //then
    assert!(res.is_err());
}

#[test]
fn is_open_true() {
    //given
    let fw = new_filter_wheel();
    //when
    let res = fw.is_open();
    //then
    assert!(res.unwrap());
}

#[test]
fn is_open_false() {
    //given
    let camera = Camera::new("test_camera".to_owned());
    let fw = FilterWheel::new(camera);
    //when
    let res = fw.is_open();
    //then
    assert!(!res.unwrap());
}

#[test]
fn close_success() {
    //given
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().once().return_const_st(TEST_HANDLE);
    let ctx_close = CloseQHYCCD_context();
    ctx_close.expect().once().return_const_st(QHYCCD_SUCCESS);
    let camera = Camera::new("test_camera".to_owned());
    let fw = FilterWheel::new(camera);
    //when
    let res = fw.open();
    assert!(res.is_ok());

    let res = fw.close();
    //then
    assert!(res.is_ok());
}

#[test]
fn is_cfw_plugged_in_true() {
    //given
    let ctx_available = IsQHYCCDCFWPlugged_context();
    ctx_available
        .expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .once()
        .return_const_st(QHYCCD_SUCCESS);
    let fw = new_filter_wheel();
    //when
    let res = fw.is_cfw_plugged_in();
    //then
    assert!(res.unwrap());
}

#[test]
fn is_cfw_plugged_in_false() {
    //given
    let ctx_available = IsQHYCCDCFWPlugged_context();
    ctx_available
        .expect()
        .withf_st(|handle| *handle == TEST_HANDLE)
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.is_cfw_plugged_in();
    //then
    assert!(!res.unwrap());
}

#[test]
fn get_number_of_filters_success() {
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
        .once()
        .return_const_st(7.0);
    let fw = new_filter_wheel();
    //when
    let res = fw.get_number_of_filters();
    //then
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn get_number_of_filters_fail_no_filter_wheel() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| {
            *handle == TEST_HANDLE && *control == Control::CfwSlotsNum as u32
        })
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.get_number_of_filters();
    //then
    assert!(res.is_err());
}

#[test]
fn get_number_of_filters_fail_get_parameter() {
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
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.get_number_of_filters();
    //then
    assert!(res.is_err());
}

#[test]
fn get_fw_position_success() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let ctx_num = GetQHYCCDParam_context();
    ctx_num
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .once()
        .return_const_st(53.0); //ASCII for 5
    let fw = new_filter_wheel();
    //when
    let res = fw.get_fw_position();
    //then
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn get_fw_position_fail_no_filter_wheel() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.get_fw_position();
    //then
    assert!(res.is_err());
}

#[test]
fn get_fw_position_fail_get_parameter() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let ctx_num = GetQHYCCDParam_context();
    ctx_num
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.get_fw_position();
    //then
    assert!(res.is_err());
}

#[test]
fn set_fw_position_success() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let ctx_num = SetQHYCCDParam_context();
    ctx_num
        .expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::CfwPort as u32 && *value == 53.0
            //ASCII for 5
        })
        .once()
        .return_const_st(QHYCCD_SUCCESS);
    let fw = new_filter_wheel();
    //when
    let res = fw.set_fw_position(5);
    //then
    assert!(res.is_ok());
}

#[test]
fn set_fw_position_fail_no_filter_wheel() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.set_fw_position(5);
    //then
    assert!(res.is_err());
}

#[test]
fn set_fw_position_fail_set_parameter() {
    //given
    let ctx_available = IsQHYCCDControlAvailable_context();
    ctx_available
        .expect()
        .withf_st(|handle, control| *handle == TEST_HANDLE && *control == Control::CfwPort as u32)
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let ctx_num = SetQHYCCDParam_context();
    ctx_num
        .expect()
        .withf_st(|handle, control, value| {
            *handle == TEST_HANDLE && *control == Control::CfwPort as u32 && *value == 53.0
            //ASCII for 5
        })
        .once()
        .return_const_st(QHYCCD_ERROR);
    let fw = new_filter_wheel();
    //when
    let res = fw.set_fw_position(5);
    //then
    assert!(res.is_err());
}
