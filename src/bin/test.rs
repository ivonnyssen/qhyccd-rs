use qhyccd_rs::FilterWheel;
use qhyccd_rs::Sdk;
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

    let camera = sdk.filter_wheels().last().expect("no camera found");
    trace!(camera = ?camera);

    camera.open().expect("open_camera failed");
    trace!(camera_open = ?camera.is_open());

    let res = camera
        .get_number_of_filters()
        .expect("get_number_of_filters failed");
    trace!(get_number_of_filters = ?res);

    camera.set_fw_position(5).expect("set_fw_position failed");

    loop {
        let res = camera.get_fw_position().expect("get_cfw_status failed");
        trace!(get_cfw_status = ?res);
    }
}
