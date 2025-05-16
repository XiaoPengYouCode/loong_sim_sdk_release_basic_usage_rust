use ndarray::{Array1, array};
use std::io::Error;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use tracing::{debug, error, info};

pub mod ctrl;
pub mod sens;

use crate::param::LoongManiParam;
use crate::sdk::ctrl::CtrlData;
use crate::sdk::sens::SensData;

pub struct LoongManiSdk {
    socket: UdpSocket,
    target_addr: SocketAddrV4,
    sens: SensData,
    ctrl: CtrlData,
}

impl LoongManiSdk {
    pub fn from_param(param: &LoongManiParam) -> Self {
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
            target_addr: param.target_addr().parse().unwrap(),
            sens: SensData::new(
                param.jnt_num(),
                param.finger_dof_left(),
                param.finger_dof_right(),
            ),
            ctrl: CtrlData::new(
                param.arm_dof(),
                param.finger_dof_left(),
                param.finger_dof_right(),
                param.neck_dof(),
                param.lumbar_dof(),
            ),
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
        let finger_left_data = Array1::zeros(self.ctrl().finger_dof().unwrap() as usize);
        let finger_right_data = Array1::zeros(self.ctrl().finger_dof().unwrap() as usize);

        self.ctrl_mut()
            .set_arm_cmd(arm_cmd_data.clone())
            .set_finger_left(finger_left_data.clone())
            .set_finger_right(finger_right_data.clone());
        Ok(())
    }
}

fn init_mani_socket(socket_addr_port: SocketAddrV4) -> Result<UdpSocket, Error> {
    info!("Binding to socket: {:?}", socket_addr_port);
    let socket = UdpSocket::bind(socket_addr_port)?;
    socket.set_nonblocking(true).unwrap();
    Ok(socket)
}
