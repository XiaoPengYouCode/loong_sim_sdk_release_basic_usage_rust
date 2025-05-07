// #!/usr/bin/env python3
// # coding=utf-8
// ============== ***doc description @ yyp*** ===========
// Copyright 2025 国地共建人形机器人创新中心/人形机器人（上海）有限公司, https://www.openloong.net/
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// Author: YYP
// 调用 mani sdk udp，接口定义见之
// ======================================================

use tokio::time::{Duration, interval};
use tracing::{Level, info};
use tracing_subscriber;

/// data: 2025.04.30
/// author: XiaoPengYouCode.github.com
use ndarray::prelude::*;
use openloong_sdk_basic_usage_example_rust::{
    loong_robot_param::{LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT},
    sdk::LoongManiSdk,
};

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
    let mut ticker = interval(Duration::from_millis(20));

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
