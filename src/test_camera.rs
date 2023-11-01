use super::*;
use crate::mocks::mock_libqhyccd_sys::{
    GetQHYCCDId_context, GetQHYCCDSDKVersion_context, InitQHYCCDResource_context,
    IsQHYCCDCFWPlugged_context, OpenQHYCCD_context, ReleaseQHYCCDResource_context,
    ScanQHYCCD_context, QHYCCD_SUCCESS,
};

use crate::QHYError::{GetCameraIdError, InitSDKError, ScanQHYCCDError};

#[test]
fn new_fail_null_error() {
    //given
    let s = "abc\0def".to_owned();
    //when
    let res = Camera::new(s);
    //then
    assert!(res.is_err());
}
