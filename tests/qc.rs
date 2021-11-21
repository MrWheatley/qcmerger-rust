use qcmerger_rust::qc;
use std::path::PathBuf;

#[test]
fn parse_test() {
    let qc_file = r"C:\Users\mrmon\PycharmProjects\qcmerger-rust\tests\scout\c_scout_animations.qc";
    let x = qc::QC::new(qc_file).unwrap();
    assert_eq!(x.qc_file, PathBuf::from(qc_file));

    let p_draw = &x.sequences["p_draw"];
    assert_eq!(
        &p_draw.smd,
        r"c_scout_animations_anims\c_scout_arms_skeleton.smd"
    );
    assert_eq!(&p_draw.weightlist, "test_weight");
    assert_eq!(&p_draw.layer[0], "test_layer");
    assert_eq!(&p_draw.activity, "ACT_SECONDARY_VM_DRAW");
    assert_eq!(&p_draw.start, &483usize);
    assert_eq!(&p_draw.end, &494usize);
    assert!(!&p_draw.uses_animation);

    let test_animation2 = &x.animations["test_animation2"];
    assert_eq!(&test_animation2.name, "test_animation2");
    assert_eq!(&test_animation2.weightlist, "weights_r_handposes");
    assert_eq!(
        &test_animation2.smd,
        r"c_scout_animations_anims\p_test_idle2.smd"
    );
    assert_eq!(&test_animation2.start, &275usize);
    assert_eq!(&test_animation2.end, &279usize);

    let test_layer_anim = &x.animations["test_layer_anim"];
    assert_eq!(&test_layer_anim.name, "test_layer_anim");
    assert_eq!(
        &test_layer_anim.smd,
        r"c_scout_animations_anims\test_layer.smd"
    );
    assert_eq!(&test_layer_anim.start, &1204usize);
    assert_eq!(&test_layer_anim.end, &1204usize);

    let weights_r_handposes = &x.weightlists["weights_r_handposes"];
    assert_eq!(weights_r_handposes.name, "weights_r_handposes");
}

#[test]
fn parse_test_sca() {
    let qc_file = r"C:\Users\mrmon\PycharmProjects\qcmerger-rust\tests\Pistol\c_scout_pistol.qc";
    let x = qc::QC::new(qc_file).unwrap();
    assert_eq!(x.qc_file, PathBuf::from(qc_file));

    let p_draw = &x.sequences["p_draw"];
    assert_eq!(&p_draw.smd, "p_draw.smd");
    assert_eq!(&p_draw.activity, "ACT_SECONDARY_VM_DRAW");
    assert_eq!(&p_draw.start, &6usize);
    assert_eq!(&p_draw.end, &11usize);
}

#[test]
#[should_panic]
fn parse_test_fail() {
    let _ = qc::QC::new(r"non-existent file").unwrap();
}
