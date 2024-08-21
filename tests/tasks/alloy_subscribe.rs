use myapp::app::App;
use loco_rs::{task, testing};

use loco_rs::boot::run_task;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_alloy_subscribe() {
    let boot = testing::boot_test::<App>().await.unwrap();

    assert!(
        run_task::<App>(&boot.app_context, Some(&"alloy_subscribe".to_string()), &task::Vars::default())
            .await
            .is_ok()
    );
}
