#![allow(non_snake_case)]
use qhyccd_rs::{
    close_camera, get_camera_id, get_ccd_info, get_effective_area, get_firmware_version,
    get_image_size, get_overscan_area, get_sdk_version, get_single_frame, init_camera, init_sdk,
    is_feature_supported, open_camera, release_sdk, scan_qhyccd, set_bin_mode, set_bit_mode,
    set_parameter, set_readout_mode, set_roi, set_stream_mode, start_single_frame_exposure,
    CameraFeature, CameraStreamMode,
};
use tracing::{error, trace};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sdk_version = get_sdk_version().expect("get_sdk_version failed");
    trace!(sdk_version = ?sdk_version);

    init_sdk().expect("init_sdk failed");

    let number_of_cameras = scan_qhyccd().expect("scan_qhyccd failed");
    trace!(number_of_cameras = ?number_of_cameras);

    let id = get_camera_id(0).expect("get_camera_id failed");

    let camera = open_camera(id).expect("open_camera failed");

    let fw_version = get_firmware_version(camera).expect("get_firmware_version failed");
    trace!(fw_version = ?fw_version);

    if is_feature_supported(camera, CameraFeature::CamSingleFrameMode).is_err() {
        release_sdk().expect("release_sdk failed");
        panic!("CameraFeature::CamLiveVideoMode is not supported");
    }
    trace!("CameraFeature::CamSingleFrameMode is supported");

    set_stream_mode(camera, CameraStreamMode::SingleFrameMode)
        .expect("set_camera_stream_mode failed");
    trace!(set_camera_stream_mode = ?CameraStreamMode::SingleFrameMode);

    set_readout_mode(camera, 0).expect("set_camera_read_mode failed");
    trace!(set_camera_read_mode = 0);

    init_camera(camera).expect("init_camera failed");

    let over_scan_area = get_overscan_area(camera).expect("get_camera_overscan_area failed");
    trace!(over_scan_area = ?over_scan_area);

    let effective_area = get_effective_area(camera).expect("get_camera_effective_area failed");
    trace!(effective_area = ?effective_area);

    let info = get_ccd_info(camera).expect("get_camera_ccd_info failed");
    trace!(ccd_info = ?info);

    let camera_is_color = is_feature_supported(camera, CameraFeature::CamColor).is_ok(); //this returns a BayerID if it is a color camera
    trace!(camera_is_color = ?camera_is_color);

    match is_feature_supported(camera, CameraFeature::ControlUsbTraffic) {
        Ok(_) => {
            trace!(control_usb_traffic = 10);
            set_parameter(camera, CameraFeature::ControlUsbTraffic, 255.0)
                .expect("set_camera_parameter failed");
        }
        Err(_) => {
            error!("ControlUsbTraffic is not supported");
            return;
        }
    }

    match is_feature_supported(camera, CameraFeature::ControlGain) {
        Ok(_) => {
            trace!(control_gain = 10);
            set_parameter(camera, CameraFeature::ControlGain, 10.0).expect("setting gain failed");
        }
        Err(_) => {
            error!("ControlGain is not supported");
            return;
        }
    }

    match is_feature_supported(camera, CameraFeature::ControlOffset) {
        Ok(_) => {
            trace!(control_offset = 140);
            set_parameter(camera, CameraFeature::ControlOffset, 140.0)
                .expect("setting offset failed");
        }
        Err(_) => {
            error!("ControlOffset is not supported");
            return;
        }
    }

    set_parameter(camera, CameraFeature::ControlExposure, 2000.0)
        .expect("setting exposure time failed");
    trace!(exposure_time = 2000.0);

    set_roi(camera, effective_area).expect("set_camera_roi failed");
    trace!(roi = ?effective_area);

    set_bin_mode(camera, 1, 1).expect("set_camera_bin_mode failed");
    trace!(bin_mode = "(1, 1)");

    match is_feature_supported(camera, CameraFeature::ControlTransferBit) {
        Ok(_) => {
            trace!(cam_transfer_bit = 16.0);
            set_bit_mode(camera, 16).expect("setting transfer bits to 16 failed");
        }
        Err(_) => {
            error!("setting transfer bits is not supported");
            return;
        }
    }

    trace!("beginning single frame capture");
    start_single_frame_exposure(camera).expect("start_camera_single_frame_exposure failed");

    let buffer_size = get_image_size(camera).expect("get_camera_image_size failed");

    let image = get_single_frame(camera, buffer_size).expect("get_camera_single_frame failed");
    trace!(image = ?image);

    close_camera(camera).expect("close_camera failed");
    trace!("camera closed");

    release_sdk().expect("release_sdk failed");
    trace!("sdk released");
}
