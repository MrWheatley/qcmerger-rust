use qcmerger_rust::qc::weightlist::Weightlist;

#[test]
fn parse_test() {
    let x = String::from(
        r#"$weightlist "test_weight" {
 "root" 0
 "bip_collar_L" 0
 "bip_collar_R" 0
 "bip_upperArm_L" 0
 "bip_upperArm_R" 0
 "bip_lowerArm_L" 0
 "bip_lowerArm_R" 0
 "bip_hand_L" 0
 "bip_hand_R" 0
 "effect_hand_L" 0
 "effect_hand_R" 0
 "vm_weapon_bone" 1
 "vm_weapon_bone_1" 1
 "vm_weapon_bone_2" 1
 "vm_weapon_bone_3" 1
 "vm_weapon_bone_4" 1
 "vm_weapon_bone_5" 1
 "vm_weapon_bone_6" 1
 "vm_weapon_bone_7" 1
 "vm_weapon_bone_L" 0
 "vm_weapon_bone_L_1" 0
 "vm_weapon_bone_L_2" 0
 "weapon_bone" 1
 "weapon_bone_1" 0
 "weapon_bone_2" 0
 "weapon_bone_3" 0
 "weapon_bone_4" 0
 "weapon_bone_L" 0
 "bip_thumb_0_L" 0
 "bip_thumb_0_R" 1
 "bip_thumb_1_L" 0
 "bip_thumb_1_R" 1
 "bip_thumb_2_L" 0
 "bip_thumb_2_R" 1
 "bip_index_0_L" 0
 "bip_index_0_R" 1
 "bip_index_1_L" 0
 "bip_index_1_R" 1
 "bip_index_2_L" 0
 "bip_index_2_R" 1
 "bip_middle_0_L" 0
 "bip_middle_0_R" 1
 "bip_middle_1_L" 0
 "bip_middle_1_R" 1
 "bip_middle_2_L" 0
 "bip_middle_2_R" 1
 "bip_ring_0_L" 0
 "bip_ring_0_R" 1
 "bip_ring_1_L" 0
 "bip_ring_1_R" 1
 "bip_ring_2_L" 0
 "bip_ring_2_R" 1
 "bip_pinky_0_L" 0
 "bip_pinky_0_R" 1
 "bip_pinky_1_L" 0
 "bip_pinky_1_R" 1
 "bip_pinky_2_L" 0
 "bip_pinky_2_R" 1
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Weightlist::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "test_weight");
    assert_eq!(z.start, 0usize);
    assert_eq!(z.end, 59usize);
    assert_eq!(
        z.block,
        x.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
    );
}

#[test]
#[should_panic]
fn parse_test_fail() {
    let x = String::from(
        r#"$weightlist "test_weight" "this fails" {
 "root" 0
 "bip_collar_L" 0
 "bip_collar_R" 0
 "bip_upperArm_L" 0
 "bip_upperArm_R" 0
 "bip_lowerArm_L" 0
 "bip_lowerArm_R" 0
 "bip_hand_L" 0
 "bip_hand_R" 0
 "effect_hand_L" 0
 "effect_hand_R" 0
 "vm_weapon_bone" 1
 "vm_weapon_bone_1" 1
 "vm_weapon_bone_2" 1
 "vm_weapon_bone_3" 1
 "vm_weapon_bone_4" 1
 "vm_weapon_bone_5" 1
 "vm_weapon_bone_6" 1
 "vm_weapon_bone_7" 1
 "vm_weapon_bone_L" 0
 "vm_weapon_bone_L_1" 0
 "vm_weapon_bone_L_2" 0
 "weapon_bone" 1
 "weapon_bone_1" 0
 "weapon_bone_2" 0
 "weapon_bone_3" 0
 "weapon_bone_4" 0
 "weapon_bone_L" 0
 "bip_thumb_0_L" 0
 "bip_thumb_0_R" 1
 "bip_thumb_1_L" 0
 "bip_thumb_1_R" 1
 "bip_thumb_2_L" 0
 "bip_thumb_2_R" 1
 "bip_index_0_L" 0
 "bip_index_0_R" 1
 "bip_index_1_L" 0
 "bip_index_1_R" 1
 "bip_index_2_L" 0
 "bip_index_2_R" 1
 "bip_middle_0_L" 0
 "bip_middle_0_R" 1
 "bip_middle_1_L" 0
 "bip_middle_1_R" 1
 "bip_middle_2_L" 0
 "bip_middle_2_R" 1
 "bip_ring_0_L" 0
 "bip_ring_0_R" 1
 "bip_ring_1_L" 0
 "bip_ring_1_R" 1
 "bip_ring_2_L" 0
 "bip_ring_2_R" 1
 "bip_pinky_0_L" 0
 "bip_pinky_0_R" 1
 "bip_pinky_1_L" 0
 "bip_pinky_1_R" 1
 "bip_pinky_2_L" 0
 "bip_pinky_2_R" 1
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let _ = Weightlist::parse(&mut y, 0).unwrap();
}
