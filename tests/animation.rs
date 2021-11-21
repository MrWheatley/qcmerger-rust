use qcmerger_rust::qc::animation::Animation;

#[test]
fn parse_test() {
    let x = String::from(
        r#"$animation "test_animation" "c_scout_animations_anims\p_test_idle.smd" {
fps 30
loop
weightlist "weights_r_handposes"
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Animation::parse(&mut y, 0).unwrap();
    assert_eq!(z.name, "test_animation");
    assert_eq!(z.start, 0usize);
    assert_eq!(z.end, 4usize);
    assert_eq!(z.smd, r"c_scout_animations_anims\p_test_idle.smd");
    assert_eq!(
        z.block,
        x.lines().map(|l| l.to_owned()).collect::<Vec<String>>()
    );
    assert_eq!(z.weightlist, "weights_r_handposes");
}

#[test]
#[should_panic]
fn parse_test_fail() {
    let x = String::from(
        r#"$animation "test_animation" "c_scout_animations_anims\p_test_idle.smd" "failes" {
fps 30
loop
weightlist "weights_r_handposes"
}"#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let _ = Animation::parse(&mut y, 0).unwrap();
}

#[test]
fn parse_test_single_line() {
    let x = String::from(
        "\
$animation \"test_layer_anim\" \"c_scout_animations_anims\\test_layer.smd\" loop subtract idle 0 ",
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let z = Animation::parse(&mut y, 0).unwrap();
    assert_eq!(z.smd, r"c_scout_animations_anims\test_layer.smd");
    assert_eq!(z.name, "test_layer_anim");
    assert_eq!(z.start, 0usize);
    assert_eq!(z.end, 0usize);
}

#[test]
#[should_panic]
fn parse_test_fail_single_line() {
    let x = String::from(
        r#"
$animation "test_layer_anim" "c_scout_animations_anims\test_layer.smd" {loop subtract idle 0} "#,
    );
    let mut y: Vec<String> = x.lines().map(|l| l.to_owned()).collect();
    let _ = Animation::parse(&mut y, 0).unwrap();
}
