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

    let filter_wheel = sdk.filter_wheels().last().expect("no filter_wheel found");
    trace!(filter_wheel = ?filter_wheel);

    filter_wheel.open().expect("open_filter_wheel failed");
    trace!(filter_wheel_open = ?filter_wheel.is_open());

    let res = filter_wheel
        .get_number_of_filters()
        .expect("get_number_of_filters failed");
    trace!(get_number_of_filters = ?res);

    filter_wheel
        .set_fw_position(5)
        .expect("set_fw_position failed");

    loop {
        let res = filter_wheel
            .get_fw_position()
            .expect("get_cfw_status failed");
        trace!(get_cfw_status = ?res);
    }
}
