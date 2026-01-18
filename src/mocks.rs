#![allow(
    unused,
    non_snake_case,
    clippy::too_many_arguments,
    clippy::missing_safety_doc,
    missing_docs
)]

//This file duplicates the libqhyccd-sys bindings, but with mockable functions.
//These bindings are activated by the import config for the test target.

use mockall::automock;

pub const QHYCCD_PCIE: u32 = 9;
pub const QHYCCD_WINPCAP: u32 = 8;
pub const QHYCCD_QGIGAE: u32 = 7;
pub const QHYCCD_USBSYNC: u32 = 6;
pub const QHYCCD_USBASYNC: u32 = 5;
pub const QHYCCD_COLOR: u32 = 4;
pub const QHYCCD_MONO: u32 = 3;
pub const QHYCCD_COOL: u32 = 2;
pub const QHYCCD_NOTCOO: u32 = 1;

#[cfg_attr(test, automock)]
pub mod libqhyccd_sys {
    use core::ffi::c_char;

    pub const QHYCCD_SUCCESS: u32 = 0;
    pub const QHYCCD_ERROR: u32 = u32::MAX;
    pub const QHYCCD_ERROR_F64: f64 = u32::MAX as f64;
    pub type QhyccdHandle = *const core::ffi::c_void;

    pub unsafe fn InitQHYCCDResource() -> u32 {
        unimplemented!()
    }
    pub unsafe fn ScanQHYCCD() -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDSDKVersion(
        _year: *mut u32,
        _month: *mut u32,
        _day: *mut u32,
        _subday: *mut u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDId(index: u32, id: *mut c_char) -> u32 {
        unimplemented!()
    }
    pub unsafe fn OpenQHYCCD(id: *const c_char) -> QhyccdHandle {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDFWVersion(h: QhyccdHandle, buf: *mut u8) -> u32 {
        unimplemented!()
    }
    pub unsafe fn IsQHYCCDControlAvailable(h: QhyccdHandle, controlId: u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDReadMode(h: QhyccdHandle, mode: u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDStreamMode(h: QhyccdHandle, mode: u8) -> u32 {
        unimplemented!()
    }
    pub unsafe fn InitQHYCCD(h: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDChipInfo(
        handle: QhyccdHandle,
        chipw: *mut f64,
        chiph: *mut f64,
        imagew: *mut u32,
        imageh: *mut u32,
        pixelw: *mut f64,
        pixelh: *mut f64,
        bpp: *mut u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDBitsMode(handle: QhyccdHandle, bits: u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDDebayerOnOff(handle: QhyccdHandle, onoff: bool) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDBinMode(handle: QhyccdHandle, wbin: u32, hbin: u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDResolution(
        handle: QhyccdHandle,
        x: u32,
        y: u32,
        xsize: u32,
        ysize: u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDParam(handle: QhyccdHandle, controlId: u32) -> f64 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDParamMinMaxStep(
        handle: QhyccdHandle,
        controlId: u32,
        min: *mut f64,
        max: *mut f64,
        step: *mut f64,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SetQHYCCDParam(handle: QhyccdHandle, controlId: u32, value: f64) -> u32 {
        unimplemented!()
    }
    pub unsafe fn BeginQHYCCDLive(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDMemLength(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDLiveFrame(
        handle: QhyccdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn StopQHYCCDLive(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn CloseQHYCCD(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn ReleaseQHYCCDResource() -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDOverScanArea(
        handle: QhyccdHandle,
        startx: *mut u32,
        starty: *mut u32,
        sizex: *mut u32,
        sizey: *mut u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDEffectiveArea(
        handle: QhyccdHandle,
        startx: *mut u32,
        starty: *mut u32,
        sizex: *mut u32,
        sizey: *mut u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn ExpQHYCCDSingleFrame(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDSingleFrame(
        handle: QhyccdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDNumberOfReadModes(handle: QhyccdHandle, num_modes: *mut u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDReadModeResolution(
        handle: QhyccdHandle,
        mode: u32,
        width: *mut u32,
        height: *mut u32,
    ) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDReadModeName(handle: QhyccdHandle, mode: u32, name: *mut c_char) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDReadMode(handle: QhyccdHandle, mode: *mut u32) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDModel(handle: QhyccdHandle, model: *mut c_char) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDType(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDExposureRemaining(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn CancelQHYCCDExposing(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn CancelQHYCCDExposingAndReadout(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn IsQHYCCDCFWPlugged(handle: QhyccdHandle) -> u32 {
        unimplemented!()
    }
    pub unsafe fn GetQHYCCDCFWStatus(handle: QhyccdHandle, status: *mut c_char) -> u32 {
        unimplemented!()
    }
    pub unsafe fn SendOrder2QHYCCDCFW(handle: QhyccdHandle, order: *const c_char, length: u32) -> u32 {
        unimplemented!()
    }
}
