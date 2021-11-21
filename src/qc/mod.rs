pub mod animation;
mod macros;
pub mod sequence;
pub mod weightlist;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct QC {
    // the path of the qc file
    pub qc_file: PathBuf,
    // all of the $sequences in the qc file
    pub sequences: HashMap<String, sequence::Sequence>,
    // all of the $animations in the qc file
    pub animations: HashMap<String, animation::Animation>,
    // all of the $weightlists in the qc file
    pub weightlists: HashMap<String, weightlist::Weightlist>,
}

pub enum QCCommand {
    Sequence,
    Animation,
    Weightlist,
}

pub enum QCBlock {
    Sequence(sequence::Sequence),
    Animation(animation::Animation),
    Weightlist(weightlist::Weightlist),
}

impl QC {
    pub fn new<P: AsRef<Path>>(qc_file: P) -> Result<QC> {
        let reader = BufReader::new(
            File::open(&qc_file)
                .with_context(|| format!("Failed to read: {}", &qc_file.as_ref().display()))?,
        )
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned());
        Ok(Self::parse(qc_file, reader)?)
    }

    // parses the qc file and returns a qc struct
    pub fn parse<P, I>(qc_file: P, reader: I) -> Result<QC>
    where
        P: AsRef<Path>,
        I: Iterator<Item = String>,
    {
        let mut qc_data = reader.enumerate();

        // will store an entire $sequence, $animation, or $weightlist block
        let mut block: Vec<String> = Vec::new();
        let mut qc: QC = Default::default();
        qc.qc_file = PathBuf::from(qc_file.as_ref());

        loop {
            let (idx, line) = match qc_data.next() {
                None => break,
                Some(l) => l,
            };
            // if line is empty or is a comment, skip
            if line.is_empty()
                || line.starts_with("//")
                || line.starts_with('#')
                || line.starts_with(';')
            {
                continue;
            } else if line.starts_with("/*") {
                // skip lines until there's a `*/`
                loop {
                    let line = qc_data.next();
                    if line.is_none() || line.unwrap().1.contains("*/") {
                        break;
                    }
                }
            } else if line.starts_with("$sequence") {
                if let QCBlock::Sequence(seq) =
                    Self::find_block(QCCommand::Sequence, &mut qc_data, &mut block, line, idx)?
                {
                    qc.sequences.insert(seq.name.clone(), seq);
                    // clears the block for the next qc command block
                    block.clear();
                }
            } else if line.starts_with("$animation") {
                if let QCBlock::Animation(anim) =
                    Self::find_block(QCCommand::Animation, &mut qc_data, &mut block, line, idx)?
                {
                    qc.animations.insert(anim.name.clone(), anim);
                    block.clear();
                }
            } else if line.starts_with("$weightlist") {
                if let QCBlock::Weightlist(weight) =
                    Self::find_block(QCCommand::Weightlist, &mut qc_data, &mut block, line, idx)?
                {
                    qc.weightlists.insert(weight.name.clone(), weight);
                    block.clear();
                }
            }
        }
        Ok(qc)
    }

    // finds the qc command block
    fn find_block<I: Iterator<Item = (usize, String)>>(
        qc_command: QCCommand,
        qc_data: &mut I,
        block: &mut Vec<String>,
        line: String,
        idx: usize,
    ) -> Result<QCBlock> {
        // assumes `{` must be on same line as qc command
        // see: https://developer.valvesoftware.com/wiki/$sequence
        block.push(line.to_owned());
        // checks if block is a single-line block, assumes only $animation will be single-lined
        // assumes single-lined $animation block won't have braces
        if !line.contains('{') && matches!(qc_command, QCCommand::Animation) {
            return Ok(QCBlock::Animation(animation::Animation::parse(
                &block, idx,
            )?));
        }
        loop {
            let (_, line) = match qc_data.next() {
                None => break,
                Some(l) => l,
            };
            // assumes `}` is on a line by itself
            if line.starts_with('}') {
                block.push(line);
                break;
            }
            block.push(line);
        }
        Ok(match qc_command {
            QCCommand::Sequence => QCBlock::Sequence(sequence::Sequence::parse(&block, idx)?),
            QCCommand::Animation => QCBlock::Animation(animation::Animation::parse(&block, idx)?),
            QCCommand::Weightlist => {
                QCBlock::Weightlist(weightlist::Weightlist::parse(&block, idx)?)
            }
        })
    }

    // gets all smd path in qc, assumes smds are relative
    pub fn get_smds(&self) -> Vec<PathBuf> {
        self.sequences
            .iter()
            .filter(|(_, seq)| !seq.uses_animation && !seq.smd.is_empty())
            .map(|(_, seq)| PathBuf::from(&seq.smd))
            .chain(
                self.animations
                    .iter()
                    .filter(|(_, anim)| !anim.smd.is_empty())
                    .map(|(_, anim)| PathBuf::from(&anim.smd)),
            )
            .collect::<Vec<PathBuf>>()
    }
}
