#![allow(non_snake_case)]
use std::{thread, time::Duration};

use qhyccd_rs::{
    begin_live, close_camera, end_live, get_camera_id, get_ccd_info, get_effective_area,
    get_firmware_version, get_image_size, get_live_frame, get_overscan_area, get_sdk_version,
    init_camera, init_sdk, is_feature_supported, open_camera, release_sdk, scan_qhyccd,
    set_bin_mode, set_bit_mode, set_parameter, set_readout_mode, set_roi, set_stream_mode,
    CameraFeature, CameraStreamMode,
};
use tracing::trace;
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

    if is_feature_supported(camera, CameraFeature::CamLiveVideoMode).is_err() {
        release_sdk().expect("release_sdk failed");
        panic!("CameraFeature::CamLiveVideoMode is not supported");
    }

    trace!("CameraFeature::CamLiveVideoMode is supported");
    set_readout_mode(camera, 0).expect("set_camera_read_mode failed");
    set_stream_mode(camera, CameraStreamMode::LiveMode).expect("set_camera_stream_mode failed");
    init_camera(camera).expect("init_camera failed");
    let info = get_ccd_info(camera).expect("get_camera_ccd_info failed");
    trace!(ccd_info = ?info);

    let over_scan_area = get_overscan_area(camera).expect("get_camera_overscan_area failed");
    trace!(over_scan_area = ?over_scan_area);

    let effective_area = get_effective_area(camera).expect("get_camera_effective_area failed");
    trace!(effective_area = ?effective_area);

    set_bit_mode(camera, 8).expect("set_camera_bit_mode failed");
    set_bin_mode(camera, 1, 1).expect("set_camera_bin_mode failed");

    set_roi(camera, effective_area).expect("set_camera_roi failed");
    trace!(roi = ?effective_area);
    set_parameter(camera, CameraFeature::ControlTransferBit, 8.0)
        .expect("set_camera_parameter failed");
    trace!(control_transferbit = 8.0);
    set_parameter(camera, CameraFeature::ControlExposure, 2000.0)
        .expect("set_camera_parameter failed");
    trace!(control_exposure = 2000.0);
    set_parameter(camera, CameraFeature::ControlUsbTraffic, 255.0)
        .expect("set_camera_parameter failed");
    trace!(control_usb_traffic = 255.0);
    set_parameter(camera, CameraFeature::ControlDDR, 1.0).expect("set_camera_parameter failed");
    trace!(control_ddr = 1.0);
    begin_live(camera).expect("begin_camera_live failed");
    let size = get_image_size(camera).expect("get_camera_image_size failed");
    trace!(image_size = ?size);

    for _ in 0..1000 {
        let result = get_live_frame(camera, size as usize);
        if result.is_err() {
            trace!("get_camera_live_frame returned error");
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        let image = result.unwrap();
        trace!(image = ?image);
        break;
    }
    end_live(camera).expect("end_camera_live failed");
    close_camera(camera).expect("close_camera failed");
    release_sdk().expect("release_sdk failed");
}
