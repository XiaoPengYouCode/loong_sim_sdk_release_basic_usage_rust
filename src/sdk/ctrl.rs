use byteorder::{LittleEndian, WriteBytesExt};
use log::error;
use ndarray::prelude::*;
use std::io::Error;

use crate::loong_robot_param::{
    LOONG_ARM_DOF, LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT, LOONG_LUMBAR_DOF, LOONG_NECK_DOF,
};

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum InCharge {
    ManiCtrlDisable,
    ManiCtrlEnable,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum FiltLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Disabled,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum ArmMode {
    None,
    Reset,
    LowerLimbCmdPassthrough,
    JntAxisCtrl,
    CartesianBodyFrame,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum FingerMode {
    None,
    Reset,
    LowerLimbCmdPassthrough,
    JntAxisCtrl,
    Extend, // 伸直
}

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum NeckMode {
    None,
    Reset,
    LowerLimbCmdPassthrough,
    JntAxisCtrl,
    NavigationFollow,
    LookLeftHand,
    LookRightHand,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug)]
pub enum LumbarMode {
    None,
    Reset,
    LowerLimbCmdPassthrough,
    JntAxisCtrl,
    PostCtrl,
}

pub struct CtrlData {
    in_charge: InCharge,
    filt_level: FiltLevel,
    arm_mode: ArmMode,
    finger_mode: FingerMode,
    neck_mode: NeckMode,
    lumbar_mode: LumbarMode,
    pub arm_cmd: Array2<f32>,
    arm_fm: Array2<f32>,
    finger_left: Array1<f32>,
    finger_right: Array1<f32>,
    neck_cmd: Array1<f32>,
    lumbar_cmd: Array1<f32>,
    arm_dof: i16,
    finger_dof_left: i16,
    finger_dof_right: i16,
    neck_dof: i16,
    lumbar_dof: i16,
}

impl CtrlData {
    pub fn new(
        arm_dof: i16,
        finger_dof_left: i16,
        finger_dof_right: i16,
        neck_dof: i16,
        lumbar_dof: i16,
    ) -> Self {
        let arm_cmd = array![
            [0.4, 0.3, 0.1, 0.0, 0.0, 0.0, 0.5],
            [0.2, -0.3, 0.1, 0.0, 0.0, 0.0, 0.5]
        ];
        if arm_cmd.shape()[1] != (arm_dof as usize) {
            error!("Invalid arm dof");
            panic!("Invalid arm dof");
        }
        Self {
            in_charge: InCharge::ManiCtrlEnable,
            filt_level: FiltLevel::Level2,
            arm_mode: ArmMode::None,
            finger_mode: FingerMode::None,
            neck_mode: NeckMode::LookLeftHand,
            lumbar_mode: LumbarMode::None,
            arm_cmd: arm_cmd.clone(),
            arm_fm: Array2::<f32>::zeros((2, 6).f()),
            finger_left: Array1::<f32>::zeros(finger_dof_left as usize),
            finger_right: Array1::<f32>::zeros(finger_dof_right as usize),
            neck_cmd: Array1::<f32>::zeros(neck_dof as usize),
            lumbar_cmd: Array1::<f32>::zeros(lumbar_dof as usize),
            arm_dof,
            finger_dof_left,
            finger_dof_right,
            neck_dof,
            lumbar_dof,
        }
    }
    pub fn default_loong_ctrl_data() -> Self {
        let mut default_loong_ctrl_data = Self::new(
            LOONG_ARM_DOF,
            LOONG_FINGER_DOF_LEFT,
            LOONG_FINGER_DOF_RIGHT,
            LOONG_NECK_DOF,
            LOONG_LUMBAR_DOF,
        );
        default_loong_ctrl_data
            .set_in_charge(InCharge::ManiCtrlEnable)
            .set_filt_level(FiltLevel::Level1)
            .set_arm_mode(ArmMode::CartesianBodyFrame)
            .set_finger_mode(FingerMode::JntAxisCtrl)
            .set_neck_mode(NeckMode::LookLeftHand)
            .set_lumbar_mode(LumbarMode::None)
            .set_arm_cmd(array![
                [0.4, 0.4, 0.1, 0.0, 0.0, 0.0, 0.5],
                [0.2, -0.4, 0.1, 0.0, 0.0, 0.0, 0.5]
            ])
            .set_arm_fm(Array2::zeros((2, 6)))
            .set_finger_left(Array1::zeros(LOONG_FINGER_DOF_LEFT as usize))
            .set_finger_right(Array1::zeros(LOONG_FINGER_DOF_RIGHT as usize))
            .set_neck_cmd(Array1::zeros(2))
            .set_lumbar_cmd(Array1::zeros(3));
        default_loong_ctrl_data
    }
    pub fn set_in_charge(&mut self, in_charge: InCharge) -> &mut Self {
        self.in_charge = in_charge;
        self
    }
    pub fn set_filt_level(&mut self, filt_level: FiltLevel) -> &mut Self {
        self.filt_level = filt_level;
        self
    }
    pub fn set_arm_mode(&mut self, arm_mode: ArmMode) -> &mut Self {
        self.arm_mode = arm_mode;
        self
    }
    pub fn set_finger_mode(&mut self, finger_mode: FingerMode) -> &mut Self {
        self.finger_mode = finger_mode;
        self
    }
    pub fn set_neck_mode(&mut self, neck_mode: NeckMode) -> &mut Self {
        self.neck_mode = neck_mode;
        self
    }
    pub fn set_lumbar_mode(&mut self, lumbar_mode: LumbarMode) -> &mut Self {
        self.lumbar_mode = lumbar_mode;
        self
    }
    pub fn set_arm_cmd(&mut self, arm_cmd: Array2<f32>) -> &mut Self {
        if arm_cmd.shape()[1] != self.arm_dof as usize {
            error!("Invalid arm dof");
            panic!("Invalid arm dof");
        }
        self.arm_cmd = arm_cmd.clone();
        self
    }
    pub fn set_arm_fm(&mut self, arm_fm: Array2<f32>) -> &mut Self {
        self.arm_fm = arm_fm.clone();
        self
    }
    pub fn set_finger_left(&mut self, finger_left: Array1<f32>) -> &mut Self {
        if finger_left.shape()[0] != self.finger_dof_left as usize {
            error!("Invalid finger dof");
            panic!("Invalid finger dof");
        }
        self.finger_left = finger_left.clone();
        self
    }
    pub fn set_finger_right(&mut self, finger_right: Array1<f32>) -> &mut Self {
        if finger_right.shape()[0] != self.finger_dof_right as usize {
            error!("Invalid finger dof");
            panic!("Invalid finger dof");
        }
        self.finger_right = finger_right.clone();
        self
    }
    pub fn set_neck_cmd(&mut self, neck_cmd: Array1<f32>) -> &mut Self {
        if neck_cmd.shape()[0] != self.neck_dof as usize {
            error!("Invalid neck dof");
            panic!("Invalid neck dof");
        }
        self.neck_cmd = neck_cmd.clone();
        self
    }
    pub fn set_lumbar_cmd(&mut self, lumbar_cmd: Array1<f32>) -> &mut Self {
        if lumbar_cmd.shape()[0] != self.lumbar_dof as usize {
            error!("Invalid lumbar dof");
            panic!("Invalid lumbar dof");
        }
        self.lumbar_cmd = lumbar_cmd.clone();
        self
    }
}

impl CtrlData {
    pub fn pack_data(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();

        buf.write_i16::<LittleEndian>(self.in_charge as i16)?;
        buf.write_i16::<LittleEndian>(self.filt_level as i16)?;
        buf.write_i16::<LittleEndian>(self.arm_mode as i16)?;
        buf.write_i16::<LittleEndian>(self.finger_mode as i16)?;
        buf.write_i16::<LittleEndian>(self.neck_mode as i16)?;
        buf.write_i16::<LittleEndian>(self.lumbar_mode as i16)?;

        for arm in self.arm_cmd.outer_iter() {
            for &val in arm.iter() {
                buf.write_f32::<LittleEndian>(val)?;
            }
        }

        for arm in self.arm_fm.outer_iter() {
            for &val in arm.iter() {
                buf.write_f32::<LittleEndian>(val)?;
            }
        }

        for &val in self.finger_left.iter() {
            buf.write_f32::<LittleEndian>(val)?;
        }

        for &val in self.finger_right.iter() {
            buf.write_f32::<LittleEndian>(val)?;
        }

        for &val in self.neck_cmd.iter() {
            buf.write_f32::<LittleEndian>(val)?;
        }

        for &val in self.lumbar_cmd.iter() {
            buf.write_f32::<LittleEndian>(val)?;
        }

        Ok(buf)
    }
}

impl std::fmt::Display for CtrlData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ManiSdkCtrlData")?;
        writeln!(f, "in_charge: {:?}", self.in_charge)?;
        writeln!(f, "filt_level: {:?}", self.filt_level)?;
        writeln!(f, "arm_mode: {:?}", self.arm_mode)?;
        writeln!(f, "finger_mode: {:?}", self.finger_mode)?;
        writeln!(f, "neck_mode: {:?}", self.neck_mode)?;
        writeln!(f, "lumbar_mode: {:?}", self.lumbar_mode)?;
        writeln!(f, "arm_cmd: {:?}", self.arm_cmd)?;
        writeln!(f, "arm_fm: {:?}", self.arm_fm)?;
        writeln!(f, "finger_left: {:?}", self.finger_left)?;
        writeln!(f, "finger_right: {:?}", self.finger_right)?;
        writeln!(f, "neck_cmd: {:?}", self.neck_cmd)?;
        writeln!(f, "lumbar_cmd: {:?}", self.lumbar_cmd)?;
        writeln!(f, "arm_dof: {}", self.arm_dof)?;
        writeln!(f, "finger_dof_left: {}", self.finger_dof_left)?;
        writeln!(f, "finger_dof_right: {}", self.finger_dof_right)?;
        writeln!(f, "neck_dof: {}", self.neck_dof)?;
        writeln!(f, "lumbar_dof: {}", self.lumbar_dof)?;
        Ok(())
    }
}
