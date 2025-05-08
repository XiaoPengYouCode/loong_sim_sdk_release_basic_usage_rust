use ndarray::{Array1, array};
use std::io::Error;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use tracing::{debug, error, info};

pub mod ctrl;
pub mod sens;

use self::ctrl::CtrlData;
use self::sens::SensData;
use crate::param::{LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT, LOONG_JNT_NUM, TARGET_ADDR};

pub struct LoongManiSdk {
    socket: UdpSocket,
    target_addr: SocketAddrV4,
    sens: SensData,
    ctrl: CtrlData,
}

impl LoongManiSdk {
    pub fn new(jnt_num: i16, finger_dof_left: i16, finger_dof_right: i16) -> Self {
        let socket =
            init_mani_socket("0.0.0.0:0".parse().unwrap()).expect("Failed to initialize socket");
        debug!(
            "sdk.socket.ip: {}",
            socket.local_addr().unwrap().ip().to_string()
        );
        debug!(
            "sdk.socket.port: {}",
            socket.local_addr().unwrap().port().to_string()
        );
        Self {
            socket,
            target_addr: TARGET_ADDR.parse().unwrap(),
            sens: SensData::new(jnt_num, finger_dof_left, finger_dof_right),
            ctrl: CtrlData::default_loong_ctrl_data(),
        }
    }

    pub fn sens(&self) -> &SensData {
        &self.sens
    }

    pub fn sens_mut(&mut self) -> &mut SensData {
        &mut self.sens
    }

    pub fn ctrl(&self) -> &CtrlData {
        &self.ctrl
    }

    pub fn ctrl_mut(&mut self) -> &mut CtrlData {
        &mut self.ctrl
    }
}

impl Default for LoongManiSdk {
    fn default() -> Self {
        Self::new(LOONG_JNT_NUM, LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT)
    }
}

impl LoongManiSdk {
    pub fn send(&self) -> Result<(), Error> {
        let data = self.ctrl.pack_data().unwrap();
        self.socket.send_to(&data, self.target_addr)?;
        info!("send data: {}", self.ctrl());
        Ok(())
    }

    pub fn recv(&mut self) -> Result<(), Error> {
        let mut buf = [0; 2048];
        if let Ok((size, src_addr)) = self.socket.recv_from(&mut buf) {
            debug!("src_addr: {}", src_addr.to_string());
            debug!("Received data size: {}", size);
            match self.sens.unpack_data(&buf[..size]) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to unpack data: {}", e);
                }
            }
        } else {
            error!("Failed to receive data");
        };
        Ok(())
    }

    pub fn pack_data(&mut self, _data: &[u8]) -> Result<(), Error> {
        let arm_cmd_data = array![
            [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
            [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
        ];
        let finger_left_data = Array1::zeros(LOONG_FINGER_DOF_LEFT as usize);
        let finger_right_data = Array1::zeros(LOONG_FINGER_DOF_RIGHT as usize);

        self.ctrl_mut()
            .set_arm_cmd(arm_cmd_data.clone())
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        Ok(())
    }

    pub fn handle_xyzrpy(
        &mut self,
        arm_string: &str,
        xyzrpy: Vec<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if xyzrpy.len() != 6 {
            return Err("Invalid data length".into());
        }
        let mut arm_cmd_data = self.ctrl().arm_cmd().clone();
        let row = match arm_string {
            "left" => 0_usize,
            "right" => 1_usize,
            _ => {
                error!("Invalid arm string");
                return Err("Invalid arm string".into());
            }
        };
        for (i, item) in xyzrpy.iter().enumerate() {
            arm_cmd_data[[row, i]] = *item as f32
        }
        arm_cmd_data[[row, 6]] = 0.0;
        self.ctrl_mut().set_arm_cmd(arm_cmd_data);
        self.send()?;
        Ok(())
    }

    pub fn handle_finger(
        &mut self,
        arm_string: &str,
        finger: &[f64],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if finger.len() != LOONG_FINGER_DOF_LEFT as usize {
            error!("Invalid data length of finger");
            return Err("Invalid data length of finger".into());
        }

        let finger_cmd_data = array![
            finger[0] as f32,
            finger[1] as f32,
            finger[2] as f32,
            finger[3] as f32,
            finger[4] as f32,
            finger[5] as f32,
            0 as f32
        ];

        match arm_string {
            "left" => {
                self.ctrl_mut().set_finger_left(finger_cmd_data);
            }
            "right" => {
                self.ctrl_mut().set_finger_right(finger_cmd_data);
            }
            _ => {
                error!("Invalid arm string");
                return Err("Invalid arm string".into());
            }
        }
        self.send()?;
        Ok(())
    }
}

fn init_mani_socket(socket_addr_port: SocketAddrV4) -> Result<UdpSocket, Error> {
    info!("Binding to socket: {:?}", socket_addr_port);
    let socket = UdpSocket::bind(socket_addr_port)?;
    socket.set_nonblocking(true).unwrap();
    Ok(socket)
}
