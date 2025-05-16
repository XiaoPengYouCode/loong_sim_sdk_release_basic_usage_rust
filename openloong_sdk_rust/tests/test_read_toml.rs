use openloong_sdk_rust::param::LoongManiParam;

#[test]
fn test_read_toml() {
    let param = LoongManiParam::read_from_toml().unwrap();
    assert_eq!(param.jnt_num(), 19);
    assert_eq!(param.arm_dof(), 7);
    assert_eq!(param.finger_dof_left(), 1);
    assert_eq!(param.finger_dof_right(), 1);
    assert_eq!(param.neck_dof(), 2);
    assert_eq!(param.lumbar_dof(), 3);
    assert_eq!(param.target_addr(), "192.168.1.201:8003");
}
