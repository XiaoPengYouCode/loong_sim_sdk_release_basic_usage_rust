pub mod ctrl;
pub mod sens;

use std::net::SocketAddrV4;
use std::{io::Error, net::UdpSocket};

use tracing::{error, info};

use self::ctrl::ManiSdkCtrlData;
use self::sens::ManiSdkSensData;

pub struct ManiSdk {
    socket: UdpSocket,
    sens: ManiSdkSensData,
}

impl ManiSdk {
    pub fn new(jnt_num: i32, finger_dof_left: i16, finger_dof_right: i16) -> Self {
        let socket =
            init_mani_socket("0.0.0.0:0".parse().unwrap()).expect("Failed to initialize socket");
        Self {
            socket,
            sens: ManiSdkSensData::new(
                jnt_num as usize,
                finger_dof_left as usize,
                finger_dof_right as usize,
            ),
        }
    }

    pub fn sens(&self) -> &ManiSdkSensData {
        &self.sens
    }
}

impl ManiSdk {
    pub fn send(&self, ctrl: &ManiSdkCtrlData, target_addr: SocketAddrV4) -> Result<(), Error> {
        let data = ctrl.pack_data().unwrap();
        self.socket.send_to(&data, target_addr)?;
        Ok(())
    }

    pub fn recv(&mut self) -> Result<(), Error> {
        let mut buf = [0; 2048];
        if let Ok((size, _)) = self.socket.recv_from(&mut buf) {
            info!("Received data size: {}", size);
            self.sens.unpack_data(&buf[..size]).unwrap();
        } else {
            error!("Failed to receive data");
        };
        Ok(())
    }
}

fn init_mani_socket(socket_addr_port: SocketAddrV4) -> Result<UdpSocket, Error> {
    info!("Binding to socket: {:?}", socket_addr_port);
    let socket = UdpSocket::bind(socket_addr_port)?;
    socket.set_nonblocking(true).unwrap();
    Ok(socket)
}
