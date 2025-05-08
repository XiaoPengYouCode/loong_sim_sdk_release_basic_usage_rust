use std::fmt;
use std::io::{Cursor, Error, Read};
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};
use ndarray::Array1;

use crate::param::{LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT, LOONG_JNT_NUM};

#[derive(Debug)]
pub struct SensData {
    pub data_size: i32,
    pub timestamp: f64,
    pub key: [i16; 2],
    pub plan_name: String,
    pub state: [i16; 2],
    pub joy: [f32; 4],

    pub rpy: [f32; 3],
    pub gyr: [f32; 3],
    pub acc: [f32; 3],
    pub act_j: Array1<f32>,
    pub act_w: Array1<f32>,
    pub act_t: Array1<f32>,
    pub drv_temp: Array1<i16>,
    pub drv_state: Array1<i16>,
    pub drv_err: Array1<i16>,
    pub tgt_j: Array1<f32>,
    pub tgt_w: Array1<f32>,
    pub tgt_t: Array1<f32>,

    pub act_finger_left: Array1<f32>,
    pub act_finger_right: Array1<f32>,
    pub tgt_finger_left: Array1<f32>,
    pub tgt_finger_right: Array1<f32>,

    pub act_tip_p_rpy2b: [[f32; 6]; 2],
    pub act_tip_vw2b: [[f32; 6]; 2],
    pub act_tip_fm2b: [[f32; 6]; 2],
    pub tgt_tip_p_rpy2b: [[f32; 6]; 2],
    pub tgt_tip_vw2b: [[f32; 6]; 2],
    pub tgt_tip_fm2b: [[f32; 6]; 2],

    fmts: Vec<String>,
    fmt_sizes: Vec<usize>,
}

impl SensData {
    pub fn new(jnt_num: i16, finger_dof_left: i16, finger_dof_right: i16) -> Self {
        SensData {
            data_size: 0,
            timestamp: 0.0,
            key: [0i16; 2],
            plan_name: "none".to_string(),
            state: [0; 2],
            joy: [0.0; 4],

            rpy: [0.0; 3],
            gyr: [0.0; 3],
            acc: [0.0; 3],
            act_j: Array1::<f32>::zeros(jnt_num as usize),
            act_w: Array1::<f32>::zeros(jnt_num as usize),
            act_t: Array1::<f32>::zeros(jnt_num as usize),
            drv_temp: Array1::<i16>::zeros(jnt_num as usize),
            drv_state: Array1::<i16>::zeros(jnt_num as usize),
            drv_err: Array1::<i16>::zeros(jnt_num as usize),
            tgt_j: Array1::<f32>::zeros(jnt_num as usize),
            tgt_w: Array1::<f32>::zeros(jnt_num as usize),
            tgt_t: Array1::<f32>::zeros(jnt_num as usize),

            act_finger_left: Array1::<f32>::zeros(finger_dof_left as usize),
            act_finger_right: Array1::<f32>::zeros(finger_dof_right as usize),
            tgt_finger_left: Array1::<f32>::zeros(finger_dof_left as usize),
            tgt_finger_right: Array1::<f32>::zeros(finger_dof_right as usize),

            act_tip_p_rpy2b: [[0.0; 6]; 2],
            act_tip_vw2b: [[0.0; 6]; 2],
            act_tip_fm2b: [[0.0; 6]; 2],
            tgt_tip_p_rpy2b: [[0.0; 6]; 2],
            tgt_tip_vw2b: [[0.0; 6]; 2],
            tgt_tip_fm2b: [[0.0; 6]; 2],

            fmts: vec![
                "i".to_string(),
                "d".to_string(),
                "2h".to_string(),
                "16s".to_string(),
                "2h".to_string(),
                "4f".to_string(),
                "3f".to_string(),
                "3f".to_string(),
                "3f".to_string(),
                format!("{}f", jnt_num),
                format!("{}f", jnt_num),
                format!("{}f", jnt_num),
                format!("{}h", jnt_num),
                format!("{}h", jnt_num),
                format!("{}h", jnt_num),
                format!("{}f", jnt_num),
                format!("{}f", jnt_num),
                format!("{}f", jnt_num),
                format!("{}f", finger_dof_left),
                format!("{}f", finger_dof_right),
                format!("{}f", finger_dof_left),
                format!("{}f", finger_dof_right),
                "12f".to_string(),
                "12f".to_string(),
                "12f".to_string(),
                "12f".to_string(),
                "12f".to_string(),
                "12f".to_string(),
            ],
            fmt_sizes: vec![
                size_of::<i32>(),
                size_of::<f64>(),
                2 * size_of::<i16>(),
                16 * size_of::<u8>(), // For string
                2 * size_of::<i16>(),
                4 * size_of::<f32>(),
                3 * size_of::<f32>(),
                3 * size_of::<f32>(),
                3 * size_of::<f32>(),
                jnt_num as usize * size_of::<f32>(),
                jnt_num as usize * size_of::<f32>(),
                jnt_num as usize * size_of::<f32>(),
                jnt_num as usize * size_of::<i16>(),
                jnt_num as usize * size_of::<i16>(),
                jnt_num as usize * size_of::<i16>(),
                jnt_num as usize * size_of::<f32>(),
                jnt_num as usize * size_of::<f32>(),
                jnt_num as usize * size_of::<f32>(),
                finger_dof_left as usize * size_of::<f32>(),
                finger_dof_right as usize * size_of::<f32>(),
                finger_dof_left as usize * size_of::<f32>(),
                finger_dof_right as usize * size_of::<f32>(),
                12 * size_of::<f32>(),
                12 * size_of::<f32>(),
                12 * size_of::<f32>(),
                12 * size_of::<f32>(),
                12 * size_of::<f32>(),
                12 * size_of::<f32>(),
            ],
        }
    }

    pub fn loong_sens_data_default() -> SensData {
        SensData::new(LOONG_JNT_NUM, LOONG_FINGER_DOF_LEFT, LOONG_FINGER_DOF_RIGHT)
    }

    pub fn get_fmt(&self) -> Vec<String> {
        self.fmts.clone()
    }

    pub fn get_fmt_size(&self) -> Vec<usize> {
        self.fmt_sizes.clone()
    }

    pub fn unpack_data(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut cursor = Cursor::new(buf);

        // 1. 解析基础字段
        let data_size = cursor.read_i32::<LittleEndian>()?;
        self.data_size = data_size;
        let timestamp = cursor.read_f64::<LittleEndian>()?;
        self.timestamp = timestamp;

        for k in &mut self.key {
            *k = cursor.read_i16::<LittleEndian>()?;
        }

        // 2. 解析 plan_name (固定 16 字节)
        let mut plan_name_buf = [0; 16];
        cursor.read_exact(&mut plan_name_buf)?;
        let plan_name = String::from_utf8_lossy(&plan_name_buf)
            .trim_end_matches('\0')
            .to_string();
        self.plan_name = plan_name;

        // 解析 state
        for i in &mut self.state {
            *i = cursor.read_i16::<LittleEndian>()?;
        }

        // 3. 解析joy[4]
        for i in &mut self.joy {
            *i = cursor.read_f32::<LittleEndian>()?;
        }

        // 4. 解析 rpy
        for i in &mut self.rpy {
            *i = cursor.read_f32::<LittleEndian>()?;
        }

        // 5. 解析 gyr
        for i in &mut self.gyr {
            *i = cursor.read_f32::<LittleEndian>()?;
        }

        // 6. 解析 acc
        for i in &mut self.acc {
            *i = cursor.read_f32::<LittleEndian>()?;
        }

        // 7. 解析 act_j
        let mut act_j = [0.0; 6];
        for i in &mut act_j {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let act_j = Array1::from_iter(act_j);
        self.act_j = act_j;

        // 8. 解析 act_w
        let mut act_w = [0.0; 6];
        for i in &mut act_w {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let act_w = Array1::from_iter(act_w);
        self.act_w = act_w;

        // 9. 解析 act_t
        let mut act_t = [0.0; 6];
        for i in &mut act_t {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let act_t = Array1::from_iter(act_t);
        self.act_t = act_t;

        // 10. 解析 drv_temp
        let mut drv_temp = [0; 6];
        for i in &mut drv_temp {
            *i = cursor.read_i16::<LittleEndian>()?;
        }
        let drv_temp = Array1::from_iter(drv_temp);
        self.drv_temp = drv_temp;

        // 11. 解析 drv_state
        let mut drv_state = [0; 6];
        for i in &mut drv_state {
            *i = cursor.read_i16::<LittleEndian>()?;
        }
        let drv_state = Array1::from_iter(drv_state);
        self.drv_state = drv_state;

        // 12. 解析 drv_err
        let mut drv_err = [0; 6];
        for i in &mut drv_err {
            *i = cursor.read_i16::<LittleEndian>()?;
        }
        let drv_err = Array1::from_iter(drv_err);
        self.drv_err = drv_err;

        // 13. 解析 tgt_j
        let mut tgt_j = [0.0; 6];
        for i in &mut tgt_j {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let tgt_j = Array1::from_iter(tgt_j);
        self.tgt_j = tgt_j;

        // 14. 解析 tgt_w
        let mut tgt_w = [0.0; 6];
        for i in &mut tgt_w {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let tgt_w = Array1::from_iter(tgt_w);
        self.tgt_w = tgt_w;

        // 15. 解析 tgt_t
        let mut tgt_t = [0.0; 6];
        for i in &mut tgt_t {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let tgt_t = Array1::from_iter(tgt_t);
        self.tgt_t = tgt_t;

        //解析 act_finger_left
        let mut act_finger_left = [0.0; 6];
        for i in &mut act_finger_left {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let act_finger_left = Array1::from_iter(act_finger_left);
        self.act_finger_left = act_finger_left;

        //解析 act_finger_right
        let mut act_finger_right = [0.0; 6];
        for i in &mut act_finger_right {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let act_finger_right = Array1::from_iter(act_finger_right);
        self.act_finger_right = act_finger_right;

        // 解析tgt_finger_left
        let mut tgt_finger_left = [0.0; 6];
        for i in &mut tgt_finger_left {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let tgt_finger_left = Array1::from_iter(tgt_finger_left);
        self.tgt_finger_left = tgt_finger_left;

        // 解析tgt_finger_right
        let mut tgt_finger_right = [0.0; 6];
        for i in &mut tgt_finger_right {
            *i = cursor.read_f32::<LittleEndian>()?;
        }
        let tgt_finger_right = Array1::from_iter(tgt_finger_right);
        self.tgt_finger_right = tgt_finger_right;

        // 解析 act_tip_p_rpy2b
        for row in &mut self.act_tip_p_rpy2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        // 解析 act_tip_vw2b
        for row in &mut self.act_tip_vw2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        // 解析 act_tip_fm2b
        for row in &mut self.act_tip_fm2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        // 解析 tgt_tip_p_rpy2b
        for row in &mut self.tgt_tip_p_rpy2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        // 解析 tgt_tip_vw2b
        for row in &mut self.tgt_tip_vw2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        // 解析tgt_tip_fm2b
        for row in &mut self.tgt_tip_fm2b {
            for item in row {
                *item = cursor.read_f32::<LittleEndian>()?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for SensData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ManiSdkSensData:")?;
        writeln!(f, "data_size = {:?}", self.data_size)?;
        writeln!(f, "timestamp = {:?}", self.timestamp)?;
        writeln!(f, "key = {:?}", self.key)?;
        writeln!(f, "plan_name = {:?}", self.plan_name)?;
        writeln!(f, "state = {:?}", self.state)?;
        writeln!(f, "joy = {:?}", self.joy)?;
        writeln!(f, "rpy = {:?}", self.rpy)?;
        writeln!(f, "gyr = {:?}", self.gyr)?;
        writeln!(f, "acc = {:?}", self.acc)?;
        writeln!(f, "act_j = {:?}", self.act_j)?;
        writeln!(f, "act_w = {:?}", self.act_w)?;
        writeln!(f, "act_t = {:?}", self.act_t)?;
        writeln!(f, "drv_temp = {:?}", self.drv_temp)?;
        writeln!(f, "drv_state = {:?}", self.drv_state)?;
        writeln!(f, "drv_err = {:?}", self.drv_err)?;
        writeln!(f, "tgt_j = {:?}", self.tgt_j)?;
        writeln!(f, "tgt_w = {:?}", self.tgt_w)?;
        writeln!(f, "tgt_t = {:?}", self.tgt_t)?;
        writeln!(f, "act_finger_left = {:?}", self.act_finger_left)?;
        writeln!(f, "act_finger_right = {:?}", self.act_finger_right)?;
        writeln!(f, "tgt_finger_left = {:?}", self.tgt_finger_left)?;
        writeln!(f, "tgt_finger_right = {:?}", self.tgt_finger_right)?;
        writeln!(f, "act_tip_p_rpy2b = {:?}", self.act_tip_p_rpy2b)?;
        writeln!(f, "act_tip_vw2b = {:?}", self.act_tip_vw2b)?;
        writeln!(f, "act_tip_fm2b = {:?}", self.act_tip_fm2b)?;
        writeln!(f, "tgt_tip_p_rpy2b = {:?}", self.tgt_tip_p_rpy2b)?;
        writeln!(f, "tgt_tip_vw2b = {:?}", self.tgt_tip_vw2b)?;
        writeln!(f, "tgt_tip_fm2b = {:?}", self.tgt_tip_fm2b)?;
        Ok(())
    }
}
