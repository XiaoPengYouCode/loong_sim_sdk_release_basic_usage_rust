use ndarray::array;
use tracing::error;

use crate::sdk::LoongManiSdk;

impl LoongManiSdk {
    const MOVE_INCREMENT: f64 = 0.05;

    pub fn up(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 2]] += Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 2]] += Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
        Ok(())
    }

    pub fn down(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 2]] -= Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 2]] -= Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
        Ok(())
    }

    pub fn w(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 0]] += Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 0]] += Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
        Ok(())
    }

    pub fn s(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 0]] -= Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 0]] -= Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
        Ok(())
    }

    pub fn a(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 1]] += Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 1]] += Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
        Ok(())
    }

    pub fn d(&mut self, arm: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut arm_data = self.ctrl().arm_cmd();
        match arm {
            "left" => {
                arm_data[[0, 1]] -= Self::MOVE_INCREMENT as f32;
            }
            "right" => {
                arm_data[[1, 1]] -= Self::MOVE_INCREMENT as f32;
            }
            _ => return Err("Invalid arm string".into()),
        }
        println!("arm_data: {:?}", arm_data);
        self.ctrl_mut().set_arm_cmd(arm_data);
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
        self.ctrl_mut().set_arm_cmd(arm_cmd_data);
        Ok(())
    }

    pub fn handle_finger(
        &mut self,
        arm_string: &str,
        finger: &[f64],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if finger.len() != self.ctrl().finger_dof().unwrap() as usize {
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
