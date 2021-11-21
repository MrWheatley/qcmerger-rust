use qcmerger_rust::qc::sequence::Sequence;

#[test]
fn parse_test() {
    let x = String::from(
        r#"$sequence "p_draw" {
"c_scout_animations_anims\c_scout_arms_skeleton.smd"
activity "ACT_SECONDARY_VM_DRAW" 1
{ event 5004 1 "Weapon_Pistol.Draw" }
fadein 0.2
fadeout 0.2
snap
fps 22
frames 1 21
weightlist "test_weight"
addlayer "test_layer"
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Sequence::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "p_draw");
    assert_eq!(z.smd, r"c_scout_animations_anims\c_scout_arms_skeleton.smd");
    assert_eq!(z.activity, "ACT_SECONDARY_VM_DRAW");
    assert_eq!(
        z.block,
        x.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
    );
    assert_eq!(z.layer[0], "test_layer");
    assert_eq!(z.weightlist, "test_weight");
    assert!(!z.uses_animation);
}

#[test]
#[should_panic]
fn parse_test_fail() {
    let x = String::from(
        r#"$sequence "p_draw" "hmmm" "this should fail" {
"c_scout_animations_anims\c_scout_arms_skeleton.smd"
activity "ACT_SECONDARY_VM_DRAW" 1
{ event 5004 1 "Weapon_Pistol.Draw" }
fadein 0.2
fadeout 0.2
snap
fps 22
frames 1 21
weightlist "test_weight"
addlayer "test_layer"
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let _ = Sequence::parse(&mut y, 0).unwrap();
}

#[test]
fn parse_test_sca() {
    let x = String::from(
        r#"$sequence "p_draw" "p_draw.smd" {
fps 30
activity "ACT_SECONDARY_VM_DRAW" 1
snap
{ event 5004 1 "Weapon_Pistol.Draw" }
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Sequence::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "p_draw");
    assert_eq!(z.smd, "p_draw.smd");
    assert_eq!(z.activity, "ACT_SECONDARY_VM_DRAW");
    assert!(z.layer.is_empty());
}

#[test]
fn parse_test_uses_animation() {
    let x = String::from(
        r#"$sequence "p_draw" {
"p_draw_anim"
activity "ACT_SECONDARY_VM_DRAW" 1
{ event 5004 1 "Weapon_Pistol.Draw" }
fadein 0.2
fadeout 0.2
snap
fps 22
frames 1 21
weightlist "test_weight"
addlayer "test_layer"
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Sequence::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "p_draw");
    assert_eq!(z.smd, "p_draw_anim");
    assert_eq!(z.activity, "ACT_SECONDARY_VM_DRAW");
    assert_eq!(
        z.block,
        x.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
    );
    assert_eq!(z.layer[0], "test_layer");
    assert_eq!(z.weightlist, "test_weight");
    assert_eq!(z.start, 0usize);
    assert_eq!(z.end, 11usize);
    assert!(z.uses_animation);
}

#[test]
fn parse_test_sca_uses_animation() {
    let x = String::from(
        r#"$sequence "p_draw" "p_draw_animation" {
fps 30
activity "ACT_SECONDARY_VM_DRAW" 1
snap
{ event 5004 1 "Weapon_Pistol.Draw" }
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Sequence::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "p_draw");
    assert_eq!(z.smd, "p_draw_animation");
    assert_eq!(z.activity, "ACT_SECONDARY_VM_DRAW");
    assert_eq!(z.start, 0usize);
    assert_eq!(z.end, 5usize);
    assert!(z.layer.is_empty());
    assert!(z.uses_animation);
}
