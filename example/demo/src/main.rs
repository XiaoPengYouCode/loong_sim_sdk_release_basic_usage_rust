/// data: 2025.04.30
/// author: XiaoPengYouCode.github.com
use ndarray::prelude::*;
use tokio::time::{Duration, interval};
use tracing::{Level, info};

use openloong_sdk_rust::{param::LoongManiParam, sdk::LoongManiSdk};

// use tokio interval to control the loop rate
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let param = LoongManiParam::read_from_toml().unwrap();
    let mut sdk = LoongManiSdk::from_param(&param);
    let mut arm_cmd_data = array![
        [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
        [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
    ];
    let finger_dof = sdk.ctrl().finger_dof().ok_or("Failed to get finger dof")? as usize;
    let mut finger_left_data = Array1::<f32>::zeros(finger_dof);
    let mut finger_right_data = Array1::<f32>::zeros(finger_dof);

    // use tokio interval to control the loop rate
    let duration = Duration::from_millis(20);
    let dt = duration.as_secs_f64();
    tracing::debug!("dt = {}", dt);
    let mut ticker = interval(duration);

    for i in 0..1000 {
        info!("frame count: {}", i);
        info!("dt: {}", dt);
        arm_cmd_data[[0, 0]] = 0.4 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[0, 2]] = 0.1 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[1, 0]] = 0.2 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        finger_left_data[0] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        finger_right_data[0] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        sdk.ctrl_mut()
            .set_arm_cmd(arm_cmd_data.clone())
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        sdk.send().unwrap();
        sdk.recv().unwrap();
        info!("{}", sdk.sens());
        ticker.tick().await;
    }

    Ok(())
}
