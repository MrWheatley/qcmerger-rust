use crate::qc::QC;
use crate::{dequote, dequote_next, dequote_nth};
use anyhow::{bail, Result};

// for $animation
#[derive(Debug, Default, Clone)]
pub struct Animation {
    // the name of the $animation block
    pub name: String,
    // the smd path
    pub smd: String,
    // the whole $animation block, includes `$animation` to `}`
    pub block: Vec<String>,
    // the $weightlist it uses
    pub weightlist: String,
    // the index in the block where the start the $animation is found, add one to get line number
    pub start: usize,
    // the index in the block where the `}` is found, add one to get the line number
    pub end: usize,
}

impl Animation {
    pub fn parse(block: &Vec<String>, idx: usize) -> Result<Animation> {
        let mut animation: Animation = Default::default();

        animation.start = idx;
        animation.end = idx + block.len() - 1;

        for line in block {
            let mut line_split = line.split_whitespace();
            // gets $animation name and smd path if it's in sca format
            if line.starts_with("$animation") {
                animation.name = dequote_nth!(line_split, 1);
                match line_split.clone().count() {
                    1 => {}
                    2 => {
                        animation.smd = dequote_next!(line_split);
                    }
                    _ => {
                        if line.contains('{') {
                            bail!("[$animation Error] Weird line at {}: `{}`", idx + 1, line);
                        } else {
                            animation.smd = dequote_next!(line_split);
                        }
                    }
                }
            } else if line.starts_with("weightlist") {
                animation.weightlist = dequote_nth!(line_split, 1);
            } else if line.contains(".smd") {
                animation.smd = dequote!(line);
            }
        }
        animation.block = block.clone();
        Ok(animation)
    }
}

impl QC {
    pub fn append_animation<T: AsRef<str>>(
        &mut self,
        other_qc: &Self,
        anim: T,
        qc_data: &mut Vec<String>,
    ) -> Result<Animation> {
        let animation_rev = other_qc.animations[anim.as_ref()].block.iter().rev();
        if !self.animations.is_empty() {
            animation_rev.for_each(|line| {
                qc_data.insert(
                    self.animations
                        .iter()
                        .min_by(|(_, x), (_, y)| x.start.cmp(&y.start))
                        .unwrap()
                        .1
                        .start,
                    line.to_owned(),
                )
            });
        } else if !self.weightlists.is_empty() {
            animation_rev.for_each(|line| {
                qc_data.insert(
                    self.weightlists
                        .iter()
                        .max_by(|(_, x), (_, y)| x.end.cmp(&y.end))
                        .unwrap()
                        .1
                        .end
                        + 1,
                    line.to_owned(),
                )
            });
        } else if !self.sequences.is_empty() {
            animation_rev.for_each(|line| {
                qc_data.insert(
                    self.sequences
                        .iter()
                        .min_by(|(_, x), (_, y)| x.start.cmp(&y.start))
                        .unwrap()
                        .1
                        .start,
                    line.to_owned(),
                )
            });
        }

        *self = Self::parse(&self.qc_file.clone(), qc_data.clone().into_iter())?;
        Ok(other_qc.animations[anim.as_ref()].clone())
    }
}
