use std::ffi::c_char;

pub const QHYCCD_PCIE: u32 = 9;
pub const QHYCCD_WINPCAP: u32 = 8;
pub const QHYCCD_QGIGAE: u32 = 7;
pub const QHYCCD_USBSYNC: u32 = 6;
pub const QHYCCD_USBASYNC: u32 = 5;
pub const QHYCCD_COLOR: u32 = 4;
pub const QHYCCD_MONO: u32 = 3;
pub const QHYCCD_COOL: u32 = 2;
pub const QHYCCD_NOTCOO: u32 = 1;
pub const QHYCCD_SUCCESS: u32 = 0;
pub const QHYCCD_ERROR: u32 = 0xFFFFFFFF;

pub type QhyccdHandle = *const std::ffi::c_void;

#[link(name = "qhyccd", kind = "static")]
extern "C" {

    pub fn InitQHYCCDResource() -> u32;
    pub fn ScanQHYCCD() -> u32;
    pub fn GetQHYCCDSDKVersion(
        year: *mut u32,
        month: *mut u32,
        day: *mut u32,
        subday: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDId(index: u32, id: *mut c_char) -> u32;
    pub fn OpenQHYCCD(id: *const c_char) -> QhyccdHandle;
    pub fn GetQHYCCDFWVersion(h: QhyccdHandle, buf: *mut u8) -> u32;
    pub fn IsQHYCCDControlAvailable(h: QhyccdHandle, controlId: u32) -> u32;
    pub fn SetQHYCCDReadMode(h: QhyccdHandle, mode: u32) -> u32;
    pub fn SetQHYCCDStreamMode(h: QhyccdHandle, mode: u8) -> u32;
    pub fn InitQHYCCD(h: QhyccdHandle) -> u32;
    pub fn GetQHYCCDChipInfo(
        handle: QhyccdHandle,
        chipw: *mut f64,
        chiph: *mut f64,
        imagew: *mut u32,
        imageh: *mut u32,
        pixelw: *mut f64,
        pixelh: *mut f64,
        bpp: *mut u32,
    ) -> u32;
    pub fn SetQHYCCDBitsMode(handle: QhyccdHandle, bits: u32) -> u32;
    pub fn SetQHYCCDDebayerOnOff(handle: QhyccdHandle, onoff: bool) -> u32;
    pub fn SetQHYCCDBinMode(handle: QhyccdHandle, wbin: u32, hbin: u32) -> u32;
    pub fn SetQHYCCDResolution(handle: QhyccdHandle, x: u32, y: u32, xsize: u32, ysize: u32)
        -> u32;
    pub fn SetQHYCCDParam(handle: QhyccdHandle, controlId: u32, value: f64) -> u32;
    pub fn BeginQHYCCDLive(handle: QhyccdHandle) -> u32;
    pub fn GetQHYCCDMemLength(handle: QhyccdHandle) -> u32;
    pub fn GetQHYCCDLiveFrame(
        handle: QhyccdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
    pub fn StopQHYCCDLive(handle: QhyccdHandle) -> u32;
    pub fn CloseQHYCCD(handle: QhyccdHandle) -> u32;
    pub fn ReleaseQHYCCDResource() -> u32;
    pub fn GetQHYCCDOverScanArea(
        handle: QhyccdHandle,
        startx: *mut u32,
        starty: *mut u32,
        sizex: *mut u32,
        sizey: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDEffectiveArea(
        handle: QhyccdHandle,
        startx: *mut u32,
        starty: *mut u32,
        sizex: *mut u32,
        sizey: *mut u32,
    ) -> u32;
    pub fn ExpQHYCCDSingleFrame(handle: QhyccdHandle) -> u32;
    pub fn GetQHYCCDSingleFrame(
        handle: QhyccdHandle,
        w: *mut u32,
        h: *mut u32,
        bpp: *mut u32,
        channels: *mut u32,
        imgdata: *mut u8,
    ) -> u32;
    pub fn GetQHYCCDNumberOfReadModes(handle: QhyccdHandle, num_modes: *mut u32) -> u32;
    pub fn GetQHYCCDReadModeResolution(
        handle: QhyccdHandle,
        mode: u32,
        width: *mut u32,
        height: *mut u32,
    ) -> u32;
    pub fn GetQHYCCDReadModeName(handle: QhyccdHandle, mode: u32, name: *mut c_char) -> u32;
    pub fn GetQHYCCDReadMode(handle: QhyccdHandle, mode: *mut u32) -> u32;
    pub fn GetQHYCCDModel(handle: QhyccdHandle, model: *mut c_char) -> u32;
    pub fn GetQHYCCDType(handle: QhyccdHandle) -> u32;
    pub fn GetQHYCCDExposureRemaining(handle: QhyccdHandle) -> u32;
    pub fn CancelQHYCCDExposing(handle: QhyccdHandle) -> u32;
    pub fn CancelQHYCCDExposingAndReadout(handle: QhyccdHandle) -> u32;
}
