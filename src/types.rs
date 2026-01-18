#[derive(Debug, PartialEq)]
/// Stream mode used in `set_stream_mode`
pub enum StreamMode {
    /// Long exposure mode
    SingleFrameMode = 0,
    /// Live video mode
    LiveMode = 1,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
/// this struct is returned from `is_control_available` when used with `Control::CamColor`
pub enum BayerMode {
    GBRG = 1,
    GRBG = 2,
    BGGR = 3,
    RGGB = 4,
}

impl TryFrom<u32> for BayerMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == BayerMode::GBRG as u32 => Ok(BayerMode::GBRG),
            x if x == BayerMode::GRBG as u32 => Ok(BayerMode::GRBG),
            x if x == BayerMode::BGGR as u32 => Ok(BayerMode::BGGR),
            x if x == BayerMode::RGGB as u32 => Ok(BayerMode::RGGB),
            _ => Err(()),
        }
    }
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
