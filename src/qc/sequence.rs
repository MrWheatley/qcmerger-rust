use crate::qc::QC;
use crate::{dequote, dequote_next, dequote_nth};
use anyhow::{bail, Result};

// for $sequence
#[derive(Debug, Default, Clone)]
pub struct Sequence {
    // the name of the $sequence block
    pub name: String,
    // the activity of the $sequence
    pub activity: String,
    // the whole $sequence block, includes `$sequence` to `}`
    pub block: Vec<String>,
    // the smd file or an $animation
    pub smd: String,
    // true == uses $animation, false == uses smd file
    pub uses_animation: bool,
    // eg `addlayer`, `blendlayer`
    pub layer: Vec<String>,
    // the $weightlist it uses
    pub weightlist: String,
    // the index in the block where the start the $sequence is found, add one to get line number
    pub start: usize,
    // the index in the block where the `}` is found, add one to get the line number
    pub end: usize,
}

impl Sequence {
    pub fn parse(block: &Vec<String>, idx: usize) -> Result<Sequence> {
        let mut sequence: Sequence = Default::default();

        sequence.start = idx;
        sequence.end = idx + block.len() - 1;

        for line in block {
            let mut line_split = line.split_whitespace();
            // gets $sequence name and smd path if it's in sca format
            if line.starts_with("$sequence") {
                sequence.name = dequote_nth!(line_split, 1);
                match line_split.clone().count() {
                    1 => {} // not sca format,
                    2 => {
                        let smd: String = dequote_next!(line_split);
                        if smd.contains(".smd") {
                            sequence.smd = smd;
                        } else {
                            sequence.smd = smd;
                            sequence.uses_animation = true;
                        }
                    }
                    _ => bail!("[$sequence Error] Weird line at {}: `{}`", idx + 1, line),
                }
            } else if line.starts_with("activity") {
                sequence.activity = dequote_nth!(line_split, 1);
            } else if line.starts_with("addlayer") || line.starts_with("blendlayer") {
                sequence.layer.push(dequote_nth!(line_split, 1));
            } else if line.starts_with("weightlist") {
                sequence.weightlist = dequote_nth!(line_split, 1);
            } else if line.starts_with('"') && line.ends_with('"')
                || line.starts_with('\'') && line.ends_with('\'')
            {
                let smd: String = dequote!(line);
                if line.contains(".smd") {
                    sequence.smd = smd;
                } else {
                    sequence.smd = smd;
                    sequence.uses_animation = true;
                }
            }
        }
        sequence.block = block.clone();
        Ok(sequence)
    }
}

impl QC {
    // replaces self (base qc) with other in qc_data and updates self
    // other_qc == "replace with"
    pub fn replace_sequence<T: AsRef<str>>(
        &mut self,
        other_qc: &Self,
        seq: T,
        // data of the qc file that will be the output
        qc_data: &mut Vec<String>,
    ) -> Result<Sequence> {
        if other_qc.sequences[seq.as_ref()].activity != self.sequences[seq.as_ref()].activity {
            bail!(
                "[$sequence Error] Activities don't match: `{}` != `{}`",
                other_qc.sequences[seq.as_ref()].activity,
                self.sequences[seq.as_ref()].activity,
            );
        }
        // deletes the other sequence in qc_data
        for _ in 0..=self.sequences[seq.as_ref()].end - self.sequences[seq.as_ref()].start {
            qc_data.remove(self.sequences[seq.as_ref()].start);
        }
        // inserts self sequence where other_seq was
        other_qc.sequences[seq.as_ref()]
            .block
            .iter()
            .rev()
            .for_each(|line| qc_data.insert(self.sequences[seq.as_ref()].start, line.to_owned()));

        // updates self qc to have correct line numbers
        *self = Self::parse(&self.qc_file.clone(), qc_data.clone().into_iter())?;
        Ok(other_qc.sequences[seq.as_ref()].clone())
    }

    // appends sequence and updates
    pub fn append_sequence<T: AsRef<str>>(
        &mut self,
        other_qc: &Self,
        seq: T,
        qc_data: &mut Vec<String>,
    ) -> Result<Sequence> {
        other_qc.sequences[seq.as_ref()]
            .block
            .iter()
            .rev()
            .for_each(|line| {
                qc_data.insert(
                    self.sequences
                        .iter()
                        .max_by(|(_, x), (_, y)| x.end.cmp(&y.end))
                        .unwrap()
                        .1
                        .end
                        + 1,
                    line.to_owned(),
                )
            });

        *self = Self::parse(&self.qc_file.clone(), qc_data.clone().into_iter())?;
        Ok(other_qc.sequences[seq.as_ref()].clone())
    }
}
