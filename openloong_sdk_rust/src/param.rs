#[derive(serde::Deserialize)]
pub struct LoongManiParam {
    jnt_num: i16,
    arm_dof: i16,
    finger_dof_left: i16,
    finger_dof_right: i16,
    neck_dof: i16,
    lumbar_dof: i16,
    target_addr: String,
}

impl LoongManiParam {
    pub fn jnt_num(&self) -> i16 {
        self.jnt_num
    }
    pub fn arm_dof(&self) -> i16 {
        self.arm_dof
    }
    pub fn finger_dof_left(&self) -> i16 {
        self.finger_dof_left
    }
    pub fn finger_dof_right(&self) -> i16 {
        self.finger_dof_right
    }
    pub fn neck_dof(&self) -> i16 {
        self.neck_dof
    }
    pub fn lumbar_dof(&self) -> i16 {
        self.lumbar_dof
    }

    pub fn target_addr(&self) -> &str {
        &self.target_addr
    }

    pub fn read_from_toml() -> Result<Self, Box<dyn std::error::Error>> {
        const PARAM_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/param/param.toml");
        let param = std::fs::read_to_string(PARAM_PATH)?;
        let param: Self = toml::from_str(&param)?;
        Ok(param)
    }
}
