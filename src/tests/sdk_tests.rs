use std::ffi::{c_char, CStr};

use crate::mocks::mock_libqhyccd_sys::{
    CloseQHYCCD_context, GetQHYCCDId_context, GetQHYCCDSDKVersion_context,
    InitQHYCCDResource_context, IsQHYCCDCFWPlugged_context, OpenQHYCCD_context,
    ReleaseQHYCCDResource_context, ScanQHYCCD_context, QHYCCD_ERROR, QHYCCD_SUCCESS,
};
use crate::*;

use crate::QHYError::{GetCameraIdError, InitSDKError, ScanQHYCCDError};

#[cfg(not(feature = "simulation"))]
fn new_sdk() -> Sdk {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(2_u32);
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
        .times(2)
        .returning_st(|handle| match handle {
            ADDR1 => QHYCCD_SUCCESS,
            ADDR2 => QHYCCD_ERROR,
            _ => panic!("invalid handle"),
        });
    let ctx_close = CloseQHYCCD_context();
    ctx_close.expect().times(2).return_const_st(QHYCCD_SUCCESS);
    Sdk::new().unwrap()
}

#[test]
#[cfg(not(feature = "simulation"))]
fn new_success() {
    //given
    //when
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release
        .expect()
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let sdk = new_sdk();
    //then
    assert_eq!(sdk.cameras().count(), 2);
    assert!(sdk.cameras().last().is_some());
}

#[test]
#[cfg(not(feature = "simulation"))]
fn version_success() {
    //given
    let ctx_version = GetQHYCCDSDKVersion_context();
    ctx_version
        .expect()
        .times(1)
        .returning_st(|year, month, day, subday| unsafe {
            *year = 21;
            *month = 1;
            *day = 1;
            *subday = 0;
            QHYCCD_SUCCESS
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release
        .expect()
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let sdk = new_sdk();

    //when
    let version = sdk.version().unwrap();
    //then
    assert_eq!(
        version,
        SDKVersion {
            year: 21,
            month: 1,
            day: 1,
            subday: 0
        }
    )
}

#[test]
#[cfg(not(feature = "simulation"))]
fn version_fail() {
    //given
    let ctx_version = GetQHYCCDSDKVersion_context();
    ctx_version
        .expect()
        .times(1)
        .returning_st(|year, month, day, subday| unsafe {
            *year = 21;
            *month = 1;
            *day = 1;
            *subday = 0;
            QHYCCD_ERROR
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release
        .expect()
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    let sdk = new_sdk();

    //when
    let version = sdk.version();
    //then
    assert!(version.is_err())
}

#[test]
#[cfg(not(feature = "simulation"))]
fn filter_wheels_success() {
    //given
    //filter wheels context is set up in new_sdk()
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release
        .expect()
        .times(1)
        .return_const_st(QHYCCD_SUCCESS);
    //when
    let sdk = new_sdk();
    //then
    assert_eq!(sdk.filter_wheels().count(), 1);
    assert!(sdk.filter_wheels().last().is_some());
}

#[test]
#[cfg(not(feature = "simulation"))]
fn new_init_fail() {
    //given
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_ERROR);
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let res = Sdk::new();
    //then
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
#[cfg(not(feature = "simulation"))]
fn new_scan_fail() {
    //given
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(QHYCCD_ERROR);
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let res = Sdk::new();
    //then
    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), ScanQHYCCDError.to_string());
}

#[test]
#[cfg(not(feature = "simulation"))]
fn new_get_id_fail() {
    //given
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(2_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id.expect().times(1).return_const_st(QHYCCD_ERROR);
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let res = Sdk::new();
    //then
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
#[cfg(not(feature = "simulation"))]
fn new_get_id_invalid_utf8_fail() {
    //given
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(1_u32);
    let ctx_id = GetQHYCCDId_context();
    ctx_id
        .expect()
        .times(1)
        .returning_st(|index, c_id| match index {
            0 => unsafe {
                let cam_id = b"\xc3\x28\0";
                c_id.copy_from(cam_id.as_ptr() as *const c_char, cam_id.len());

                QHYCCD_SUCCESS
            },
            _ => panic!("too many calls"),
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let res = Sdk::new();
    //then
    assert!(res.is_err());
}

#[test]
#[cfg(not(feature = "simulation"))]
fn new_with_broken_filter_wheel() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(2_u32);
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
    let ctx_open = OpenQHYCCD_context();
    ctx_open.expect().times(2).returning_st(|c_id| {
        match unsafe { CStr::from_ptr(c_id) }.to_str() {
            Ok(id) => match id {
                "QHY178M-222b16468c5966524" => ADDR1,
                "QHY178M-222b16468c5966525" => std::ptr::null(),
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
            ADDR1 => 12345, //neither SUCCESS nor ERROR, so Err() is returned
            _ => panic!("invalid handle"),
        });
    let ctx_close = CloseQHYCCD_context();
    ctx_close
        .expect()
        .times(1)
        .returning_st(|handle| match handle {
            ADDR1 => QHYCCD_SUCCESS,
            _ => panic!("invalid handle"),
        });
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let sdk = Sdk::new().unwrap();
    //then
    assert_eq!(sdk.cameras().count(), 1);
    assert!(sdk.cameras().last().is_some());
    assert_eq!(sdk.filter_wheels().count(), 0);
    assert!(sdk.filter_wheels().last().is_none());
}

#[test]
#[cfg(not(feature = "simulation"))]
fn new_fail_close() {
    let ctx_init = InitQHYCCDResource_context();
    ctx_init.expect().times(1).return_const_st(QHYCCD_SUCCESS);
    let ctx_scan = ScanQHYCCD_context();
    ctx_scan.expect().times(1).return_const_st(1_u32);
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
            ADDR1 => QHYCCD_SUCCESS,
            _ => panic!("invalid handle"),
        });
    let ctx_close = CloseQHYCCD_context();
    ctx_close.expect().once().return_const_st(QHYCCD_ERROR);
    let ctx_release = ReleaseQHYCCDResource_context();
    ctx_release.expect().return_const_st(QHYCCD_SUCCESS);
    //when
    let sdk = Sdk::new().unwrap();
    //then
    assert_eq!(sdk.cameras().count(), 0);
    assert!(sdk.cameras().last().is_none());
    assert_eq!(sdk.filter_wheels().count(), 0);
    assert!(sdk.filter_wheels().last().is_none());
}
