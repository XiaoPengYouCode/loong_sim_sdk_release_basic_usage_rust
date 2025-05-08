/// data: 2025.04.30
/// author: XiaoPengYouCode.github.com
use ndarray::prelude::*;
use tokio::time::{Duration, interval};
use tracing::{Level, info};

use openloong_sdk_basic_usage_example_rust::{
    param::{LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT},
    sdk::LoongManiSdk,
};

// use tokio interval to control the loop rate
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let mut sdk = LoongManiSdk::default();

    let mut arm_cmd_data = array![
        [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
        [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
    ];
    let mut finger_left_data = Array1::zeros(LOONG_FINGER_DOF_LEFT as usize);
    let mut finger_right_data = Array1::zeros(LOONG_FINGER_DOF_RIGHT as usize);

    let dt = 0.02;
    let mut ticker = interval(Duration::from_millis(20)); // use tokio interval to control the loop rate

    for i in 0..1000 {
        info!("frame count: {}", i);
        arm_cmd_data[[0, 0]] = 0.4 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[0, 2]] = 0.1 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[1, 0]] = 0.2 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        finger_left_data[0] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        finger_right_data[3] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        sdk.ctrl_mut()
            .set_arm_cmd(arm_cmd_data.clone())
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        sdk.send().unwrap();
        sdk.recv().unwrap();
        info!("{}", sdk.sens());
        ticker.tick().await;
    }
}
