use super::*;
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDId_context, GetQHYCCDSDKVersion_context, InitQHYCCDResource_context,
    IsQHYCCDCFWPlugged_context, OpenQHYCCD_context, ReleaseQHYCCDResource_context,
    ScanQHYCCD_context, QHYCCD_SUCCESS,
};

use crate::QHYError::{GetCameraIdError, InitSDKError, ScanQHYCCDError};

#[test]
fn new_success() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const(2_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id
        .expect()
        .times(2)
        .returning_st(|index, c_id| match index {
            0 => unsafe {
                let cam_id = "QHY178M-222b16468c5966524\0";
                c_id.copy_from(cam_id.as_ptr() as *const c_char, cam_id.len());

                QHYCCD_SUCCESS
            },
            1 => unsafe {
                let cam_id = "QHY178M-222b16468c5966525\0";
                c_id.copy_from(cam_id.as_ptr() as *const c_char, cam_id.len());
                QHYCCD_SUCCESS
            },
            _ => panic!("too many calls"),
        });
    const ADDR1: *const core::ffi::c_void = 0xdeadbeef as *mut std::ffi::c_void;
    const ADDR2: *const core::ffi::c_void = 0xdeadbeea as *mut std::ffi::c_void;
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(2).returning_st(|c_id| {
        match unsafe { CStr::from_ptr(c_id) }.to_str() {
            Ok(id) => match id {
                "QHY178M-222b16468c5966524" => ADDR1,
                "QHY178M-222b16468c5966525" => ADDR2,
                _ => panic!("invalid id"),
            },
            Err(_) => panic!("invalid id"),
        }
    });
    let ctx_plugged = IsQHYCCDCFWPlugged_context();
    ctx_plugged
        .expect()
        .times(4)
        .returning_st(|handle| match handle {
            ADDR1 => QHYCCD_SUCCESS,
            ADDR2 => QHYCCD_ERROR,
            _ => panic!("invalid handle"),
        });

    let ctx_version = GetQHYCCDSDKVersion_context();
    ctx_version
        .expect()
        .times(1)
        .returning(|year, month, day, subday| unsafe {
            *year = 21;
            *month = 1;
            *day = 1;
            *subday = 0;
            QHYCCD_SUCCESS
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().times(1).return_const(QHYCCD_SUCCESS);
    let sdk = Sdk::new().unwrap();
    assert_eq!(sdk.cameras().count(), 2);
    assert_eq!(sdk.filter_wheels().count(), 1);
    assert!(sdk.filter_wheels().last().is_some());
    assert!(sdk.cameras().last().is_some());
    assert_eq!(
        sdk.version().unwrap(),
        SDKVersion {
            year: 21,
            month: 1,
            day: 1,
            subday: 0
        }
    )
}

#[test]
fn new_init_fail() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_ERROR);
    let res = Sdk::new();
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        InitSDKError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn new_scan_fail() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const(QHYCCD_ERROR);
    let res = Sdk::new();
    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), ScanQHYCCDError.to_string());
}

#[test]
fn new_get_id_fail() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const(2_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id.expect().times(1).return_const(QHYCCD_ERROR);
    let res = Sdk::new();
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        GetCameraIdError {
            error_code: QHYCCD_ERROR
        }
        .to_string()
    );
}

#[test]
fn new_camera_new_fail() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const(1_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id
        .expect()
        .times(1)
        .returning_st(|index, c_id| match index {
            0 => unsafe {
                let cam_id = "QHY178M-222b16468c5966524\0";
                c_id.copy_from(cam_id.as_ptr() as *const c_char, cam_id.len());

                QHYCCD_SUCCESS
            },
            _ => panic!("too many calls"),
        });
    let ctx_open = OpenQHYCCD_context();
    ctx_open
        .expect()
        .times(1)
        .returning_st(|_c_id| core::ptr::null());
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().times(1).return_const(QHYCCD_SUCCESS);
    let res = Sdk::new();
    assert!(res.is_ok());
    assert_eq!(res.unwrap().cameras().count(), 0);
}

#[test]
fn new_is_plugged_fail() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const(1_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id
        .expect()
        .times(1)
        .returning_st(|index, c_id| match index {
            0 => unsafe {
                let cam_id = "QHY178M-222b16468c5966524\0";
                c_id.copy_from(cam_id.as_ptr() as *const c_char, cam_id.len());

                QHYCCD_SUCCESS
            },
            _ => panic!("too many calls"),
        });
    const ADDR1: *const core::ffi::c_void = 0xdeadbeef as *mut std::ffi::c_void;
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(1).returning_st(|c_id| {
        match unsafe { CStr::from_ptr(c_id) }.to_str() {
            Ok(id) => match id {
                "QHY178M-222b16468c5966524" => ADDR1,
                _ => panic!("invalid id"),
            },
            Err(_) => panic!("invalid id"),
        }
    });
    let ctx_plugged = IsQHYCCDCFWPlugged_context();
    ctx_plugged
        .expect()
        .times(1)
        .returning_st(|handle| match handle {
            ADDR1 => 12345,
            _ => panic!("invalid handle"),
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().times(1).return_const(QHYCCD_SUCCESS);
    let res = Sdk::new().unwrap();
    assert_eq!(res.cameras().count(), 1);
    assert_eq!(res.filter_wheels().count(), 0);
}