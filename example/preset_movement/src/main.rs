/// data: 2025.04.30
/// author: XiaoPengYouCode.github.com
use ndarray::prelude::*;
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
    let (x, y, z, r, yaw, p, arm_angle) = (0.4, 0.4, 0.0, 0.0, 0.0, 0.0, 0.5);

    let arm_cmd_data = array![
        [x, y, z, r, yaw, p, arm_angle],
        [x, -y, z, r, yaw, p, arm_angle],
    ];
    let finger_dof = sdk.ctrl().finger_dof().ok_or("Failed to get finger dof")? as usize;
    let finger_left_data = Array1::<f32>::zeros(finger_dof);
    let finger_right_data = Array1::<f32>::zeros(finger_dof);

    let mut frame = 0_u32;
    sdk.ctrl_mut().set_arm_cmd(arm_cmd_data.clone());
    loop {
        frame += 1;
        info!("frame: {}", frame);
        sdk.ctrl_mut()
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        sdk.send()?;
    }
}
