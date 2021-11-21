use crate::dequote_nth;
use crate::qc::QC;
use anyhow::{bail, Result};

// for $weightlist
#[derive(Debug, Default, Clone)]
pub struct Weightlist {
    // the name of the $weightlist
    pub name: String,
    // the whole $weightlist block, includes `$weightlist` to `}`
    pub block: Vec<String>,
    // the index in the block where the start the $weightlist is found, add one to get line number
    pub start: usize,
    // the index in the block where the `}` is found, add one to get the line number
    pub end: usize,
}

impl Weightlist {
    pub fn parse(block: &Vec<String>, idx: usize) -> Result<Weightlist> {
        let mut weightlist: Weightlist = Default::default();

        weightlist.start = idx;
        weightlist.end = idx + block.len() - 1;

        let mut line_split = block[0].split_whitespace();
        // gets $weightlist name
        if block[0].starts_with("$weightlist") {
            match line_split.clone().count() {
                3 => weightlist.name = dequote_nth!(line_split, 1),
                _ => bail!(
                    "[$weightlist Error] Weird line at {}: `{}`",
                    idx + 1,
                    block[0]
                ),
            }
        }

        weightlist.block = block.clone();
        Ok(weightlist)
    }
}

impl QC {
    pub fn append_weightlist<T: AsRef<str>>(
        &mut self,
        other_qc: &Self,
        weight: T,
        qc_data: &mut Vec<String>,
    ) -> Result<String> {
        let weightlist_rev = other_qc.weightlists[weight.as_ref()].block.iter().rev();
        // if qc has at least one $weightlist, append weightlist next to it
        if !self.weightlists.is_empty() {
            weightlist_rev.for_each(|line| {
                qc_data.insert(
                    self.weightlists.iter().next().unwrap().1.start,
                    line.to_owned(),
                )
            });
        } else if !self.sequences.is_empty() {
            weightlist_rev.for_each(|line| {
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
        } else if !self.animations.is_empty() {
            weightlist_rev.for_each(|line| {
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
        }

        *self = Self::parse(&self.qc_file.clone(), qc_data.clone().into_iter())?;
        Ok(weight.as_ref().to_owned())
    }
}
