#[macro_export]
// takes SplitWhitespace, returns string with quotes
// e.g. dequote_nth(line_split, 2) => `$sequence "p_draw" "p_draw.smd" {`  ->  `p_draw.smd`
macro_rules! dequote_nth {
    ($line_split:ident, $num:expr) => {
        $line_split
            .nth($num)
            .unwrap()
            .chars()
            .filter(|&c| c != '"' && c != '\'')
            .collect::<String>();
    };
}

#[macro_export]
// takes SplitWhitespace, returns string with quotes
// e.g. dequote_next(line_split) => `$sequence "p_draw" {`   ->  `p_draw`
macro_rules! dequote_next {
    ($line_split:ident) => {
        $line_split
            .next()
            .unwrap()
            .chars()
            .filter(|&c| c != '"' && c != '\'')
            .collect::<String>();
    };
}

#[macro_export]
// takes SplitWhitespace, returns string with quotes
// e.g. dequote(r#""c_scout_animations_anims\c_scout_arms_skeleton.smd""#)  ->
// c_scout_animations_anims\c_scout_arms_skeleton.smd
macro_rules! dequote {
    ($line_split:ident) => {
        $line_split
            .chars()
            .filter(|&c| c != '"' && c != '\'')
            .collect::<String>();
    };
}
