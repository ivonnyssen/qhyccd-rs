#![allow(non_snake_case)]
use std::{thread, time::Duration};

use qhyccd_rs::{Control, Sdk, StreamMode};
use tracing::trace;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sdk = Sdk::new().expect("SDK::new failed");

    let sdk_version = sdk.version().expect("get_sdk_version failed");
    trace!(sdk_version = ?sdk_version);

    let camera = sdk.cameras().last().expect("no camera found");
    trace!(camera = ?camera);

    let fw_version = camera
        .get_firmware_version()
        .expect("get_firmware_version failed");
    trace!(fw_version = ?fw_version);

    if camera
        .is_control_available(Control::CamLiveVideoMode)
        .is_err()
    {
        panic!("Control::CamLiveVideoMode is not supported");
    }

    trace!("Control::CamLiveVideoMode is supported");
    camera
        .set_readout_mode(0)
        .expect("set_camera_read_mode failed");
    camera
        .set_stream_mode(StreamMode::LiveMode)
        .expect("set_camera_stream_mode failed");
    camera.init().expect("init_camera failed");
    let info = camera.get_ccd_info().expect("get_camera_ccd_info failed");
    trace!(ccd_info = ?info);

    let over_scan_area = camera
        .get_overscan_area()
        .expect("get_camera_overscan_area failed");
    trace!(over_scan_area = ?over_scan_area);

    let effective_area = camera
        .get_effective_area()
        .expect("get_camera_effective_area failed");
    trace!(effective_area = ?effective_area);

    camera.set_bit_mode(8).expect("set_camera_bit_mode failed");
    camera
        .set_bin_mode(1, 1)
        .expect("set_camera_bin_mode failed");

    camera
        .set_roi(effective_area)
        .expect("set_camera_roi failed");
    trace!(roi = ?effective_area);
    camera
        .set_parameter(Control::TransferBit, 8.0)
        .expect("set_camera_parameter failed");
    trace!(control_transferbit = 8.0);
    camera
        .set_parameter(Control::Exposure, 2000.0)
        .expect("set_camera_parameter failed");
    trace!(control_exposure = 2000.0);
    camera
        .set_parameter(Control::UsbTraffic, 255.0)
        .expect("set_camera_parameter failed");
    trace!(control_usb_traffic = 255.0);
    camera
        .set_parameter(Control::DDR, 1.0)
        .expect("set_camera_parameter failed");
    trace!(control_ddr = 1.0);
    camera.begin_live().expect("begin_camera_live failed");
    let size = camera
        .get_image_size()
        .expect("get_camera_image_size failed");
    trace!(image_size = ?size);

    for _ in 0..1000 {
        let result = camera.get_live_frame(size);
        if result.is_err() {
            trace!("get_camera_live_frame returned error");
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        let image = result.unwrap();
        trace!(image = ?image);
        break;
    }
    camera.end_live().expect("end_camera_live failed");
}
