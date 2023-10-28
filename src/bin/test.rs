use qhyccd_rs::{CameraStreamMode, Sdk};
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

    let mut cameras = sdk.cameras().expect("scan_qhyccd failed");
    trace!(number = ?cameras);

    let camera = cameras.remove(0);
    camera
        .set_stream_mode(CameraStreamMode::SingleFrameMode)
        .expect("set_camera_stream_mode failed");
    trace!(set_camera_stream_mode = ?CameraStreamMode::SingleFrameMode);

    camera
        .set_readout_mode(0)
        .expect("set_camera_read_mode failed");
    trace!(set_camera_read_mode = 0);

    let camera = camera.init().expect("init_camera failed");

    let number_of_readout_modes = camera.get_number_of_readout_modes().unwrap();
    trace!(number_of_readout_modes = ?number_of_readout_modes);
    for i in 0..number_of_readout_modes {
        let name = camera.get_readout_mode_name(i).unwrap();
        println!("{}: {}", i, name);
        let resolution = camera.get_readout_mode_resolution(i).unwrap();
        println!("{}: {}, {}", i, resolution.0, resolution.1);
    }

    let read_out_mode = camera.get_readout_mode().expect("get_readout_mode failed");
    trace!(read_out_mode = ?read_out_mode);
}
