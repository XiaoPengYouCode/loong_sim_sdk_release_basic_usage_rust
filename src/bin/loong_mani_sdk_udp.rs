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

use std::net::SocketAddrV4;
use tokio::time::{Duration, interval};
use tracing::{Level, info};
use tracing_subscriber;

/// data: 2025.04.30
/// author: XiaoPengYouCode.github.com
use ndarray::prelude::*;
use rust_impl::comm::{ManiSdkClass, ctrl::ManiSdkCtrlDataClass};

// const REMOTE_HOST_IP_PORT: &str = "192.168.1.100:8003";
const CIRCLE_HOST_IP_PORT: &str = "0.0.0.0:8003";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let jnt_num = 19;
    let arm_dof = 7;
    let finger_dof_left = 6;
    let finger_dof_right = 6;
    let neck_dof = 2;
    let lumbar_dof = 3;

    let mut ctrl = ManiSdkCtrlDataClass::new(
        arm_dof,
        finger_dof_left,
        finger_dof_right,
        neck_dof,
        lumbar_dof,
    );

    ctrl.set_in_charge(1)
        .set_filt_level(1)
        .set_arm_mode(4)
        .set_finger_mode(3)
        .set_neck_mode(5)
        .set_lumbar_mode(0)
        .set_arm_cmd(array![
            [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
            [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
        ])
        .set_arm_fm(Array2::zeros((2, 6)))
        .set_finger_left(Array1::zeros(finger_dof_left as usize))
        .set_finger_right(Array1::zeros(finger_dof_right as usize))
        .set_neck_cmd(Array1::zeros(2))
        .set_lumbar_cmd(Array1::zeros(3));

    let ip_port = CIRCLE_HOST_IP_PORT.parse::<SocketAddrV4>().unwrap();
    let mut sdk = ManiSdkClass::new(jnt_num, finger_dof_left, finger_dof_right);

    let dt = 0.02;

    let mut arm_cmd_data = array![
        [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
        [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
    ];
    let mut finger_left_data = Array1::zeros(finger_dof_left as usize);
    let mut finger_right_data = Array1::zeros(finger_dof_right as usize);

    let mut ticker = interval(Duration::from_millis(20));

    for i in 0..1000 {
        info!("frame count: {}", i);
        arm_cmd_data[[0, 0]] = 0.4 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[0, 2]] = 0.1 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        arm_cmd_data[[1, 0]] = 0.2 + 0.1 * (i as f64 * dt * 2.0).sin() as f32;
        finger_left_data[0] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        finger_right_data[3] = 40.0 + 30.0 * (i as f64 * dt * 2.0).sin() as f32;
        ctrl.set_arm_cmd(arm_cmd_data.clone())
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        sdk.send(&ctrl, ip_port).unwrap();
        info!("send data: {}", ctrl);
        sdk.recv().unwrap();
        info!("{}", sdk.sens());
        ticker.tick().await;
    }
}
