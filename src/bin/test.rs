use qhyccd_rs::{
    close_camera, get_camera_id, get_number_of_readout_modes, get_readout_mode,
    get_readout_mode_name, get_readout_mode_resolution, get_sdk_version, init_camera, init_sdk,
    open_camera, release_sdk, scan_qhyccd, set_readout_mode, set_stream_mode, CameraStreamMode,
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
    trace!(camera_id = ?id);

    let camera = open_camera(id).expect("open_camera failed");
    set_stream_mode(camera, CameraStreamMode::SingleFrameMode)
        .expect("set_camera_stream_mode failed");
    trace!(set_camera_stream_mode = ?CameraStreamMode::SingleFrameMode);

    set_readout_mode(camera, 0).expect("set_camera_read_mode failed");
    trace!(set_camera_read_mode = 0);

    init_camera(camera).expect("init_camera failed");

    let number_of_readout_modes = get_number_of_readout_modes(camera).unwrap();
    trace!(number_of_readout_modes = ?number_of_readout_modes);
    for i in 0..number_of_readout_modes {
        let name = get_readout_mode_name(camera, i).unwrap();
        println!("{}: {}", i, name);
        let resolution = get_readout_mode_resolution(camera, i).unwrap();
        println!("{}: {}, {}", i, resolution.0, resolution.1);
    }

    let read_out_mode = get_readout_mode(camera).expect("get_readout_mode failed");
    trace!(read_out_mode = ?read_out_mode);

    trace!("close_camera");
    close_camera(camera).expect("close_camera failed");
    trace!("release_sdk");
    release_sdk().expect("release_sdk failed");
}
