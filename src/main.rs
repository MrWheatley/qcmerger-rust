mod cli;
mod qc;

use anyhow::{Context, Result};
use cli::{Opt, SequenceTable};
use console::{style, Term};
use qc::QC;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    let qc_file = QC::new(&opt.file)?;
    let qc_base = QC::new(&opt.base_file)?;

    let mut table = SequenceTable::from(&qc_file);
    let mut error = String::new();
    let term = Term::stdout();

    loop {
        table.print_table(&term, &mut error, opt.dont_clear)?;
        let input = term.read_line()?;

        if input.is_empty() {
            if table.get_selected().is_empty() {
                println!("Nothing was selected, exiting...");
                exit(0);
            }
            break;
        } else if input.trim() == "exit" {
            exit(0);
        } else if input.trim() == "clear" {
            table.clear_selection();
            continue;
        }

        match cli::process_input(&qc_file, &input) {
            Ok(ok) => ok,
            Err(e) => {
                error = e.to_string();
                continue;
            }
        }
        .iter()
        .for_each(|name| table.toggle_select(name));
    }

    let selected_sequences = table
        .get_selected()
        .iter()
        .map(|name| name.to_owned())
        .collect::<Vec<String>>();

    println!(
        "Transferring: {}",
        selected_sequences
            .iter()
            .map(|name| style(name).green().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    // sequences to add to base qc file
    let mut layers = Vec::new();
    // finds any `addlayer` and `blendlayer`
    for seq in &selected_sequences {
        if &qc_file.sequences.contains_key(seq) == &true {
            if !&qc_file.sequences[seq].layer.is_empty() {
                for layer in &qc_file.sequences[seq].layer {
                    // push if it's not already selected and not in layers
                    if !selected_sequences.contains(&layer) && !layers.contains(layer) {
                        layers.push(layer.to_owned());
                    }
                }
            }
        }
    }

    let mut new_qc = qc_base.clone();
    let mut new_qc_data = BufReader::new(
        File::open(&opt.base_file)
            .with_context(|| format!("Failed to read: {}", &opt.base_file.display()))?,
    )
    .lines()
    .filter_map(|line| line.ok())
    .map(|line| line.trim().to_owned())
    .collect::<Vec<String>>();

    // might be used later to rename commands to prevent name collision
    let mut appended_weightlists = Vec::new();
    let mut appended_animations = Vec::new();
    let mut appended_sequences = Vec::new();
    let mut replaced_sequences = Vec::new();

    // replaces sequences from base_qc with those from qc_file
    // also appends $weightlist and $animation
    for seq in &selected_sequences {
        // if the sequence isn't in base_qc, then it has to be appended
        if !qc_base.sequences.contains_key(seq) {
            layers.push(seq.to_owned());
            continue;
        }
        let other_sequence = new_qc.replace_sequence(&qc_file, &seq, &mut new_qc_data)?;
        replaced_sequences.push(other_sequence.name.clone());
        if !other_sequence.weightlist.is_empty() // if it uses a $weightlist
            // if the $weightlist isn't already in new_qc
            && !new_qc.weightlists.contains_key(&other_sequence.weightlist)
        {
            appended_weightlists.push(new_qc.append_weightlist(
                &qc_file,
                &other_sequence.weightlist,
                &mut new_qc_data,
            )?);
        }
        if other_sequence.uses_animation {
            let other_animation =
                new_qc.append_animation(&qc_file, &other_sequence.smd, &mut new_qc_data)?;
            appended_animations.push(other_animation.name.clone());
            if !other_animation.weightlist.is_empty()
                && !new_qc.weightlists.contains_key(&other_animation.weightlist)
            {
                appended_weightlists.push(new_qc.append_weightlist(
                    &qc_file,
                    &other_animation.weightlist,
                    &mut new_qc_data,
                )?);
            }
        }
    }

    // appends $sequence and others from qc_file to base_qc
    for seq in &layers {
        let other_sequence = new_qc.append_sequence(&qc_file, &seq, &mut new_qc_data)?;
        appended_sequences.push(other_sequence.name.clone());
        if !other_sequence.weightlist.is_empty()
            && !new_qc.weightlists.contains_key(&other_sequence.weightlist)
        {
            appended_weightlists.push(new_qc.append_weightlist(
                &qc_file,
                &other_sequence.weightlist,
                &mut new_qc_data,
            )?);
        }
        if other_sequence.uses_animation {
            let other_animation =
                new_qc.append_animation(&qc_file, &other_sequence.smd, &mut new_qc_data)?;
            appended_animations.push(other_animation.name.clone());
            if !other_animation.weightlist.is_empty()
                && !new_qc.weightlists.contains_key(&other_animation.weightlist)
            {
                appended_weightlists.push(new_qc.append_weightlist(
                    &qc_file,
                    &other_animation.weightlist,
                    &mut new_qc_data,
                )?);
            }
        }
    }

    #[cfg(debug_assertions)]
    new_qc_data.iter().for_each(|line| println!("{}", line));

    // if output folder already exists
    if opt.output.exists() {
        println!("Output folder already exists, overwrite it? [y/n]");

        loop {
            let input = term.read_line()?;
            match input.to_lowercase().as_str() {
                "y" | "yes" => {
                    fs::remove_dir_all(&opt.output)
                        .with_context(|| format!("Failed to remove {}", opt.output.display()))?;
                    break;
                }
                "n" | "no" => break,
                _ => {
                    eprintln!("Invalid input")
                }
            }
        }
    }
    // creates output dir
    fs::create_dir_all(&opt.output)?;

    // copies base .smd files to output dir
    copy_smds(&qc_base.get_smds(), &opt.base_file, &opt.output)?;

    // copies replaced and appened smds to output dir
    let new_blocks = replaced_sequences
        .iter()
        .chain(appended_sequences.iter())
        .chain(appended_animations.iter())
        .collect::<Vec<&String>>();

    let new_seq_smds = new_blocks
        .iter()
        .filter(|&&name| qc_file.sequences.contains_key(name))
        .map(|&name| &qc_file.sequences[name])
        .filter(|seq| !seq.uses_animation && !seq.smd.is_empty())
        .map(|seq| PathBuf::from(&seq.smd))
        .collect::<Vec<PathBuf>>();

    let new_anim_smds = new_blocks
        .iter()
        .filter(|&&name| qc_file.animations.contains_key(name))
        .map(|&name| &qc_file.animations[name])
        .filter(|anim| !anim.smd.is_empty())
        .map(|anim| PathBuf::from(&anim.smd))
        .collect::<Vec<PathBuf>>();

    copy_smds(&new_seq_smds, &opt.file, &opt.output)?;
    copy_smds(&new_anim_smds, &opt.file, &opt.output)?;

    // creates output qc
    let output_qc = File::create(&opt.output.join(&opt.base_file.file_name().unwrap()))?;

    // writes to the file
    let mut write = BufWriter::new(output_qc);
    for line in new_qc_data {
        write.write_all((line + "\n").as_bytes())?;
    }

    Ok(())
}

fn copy_smds<P: AsRef<Path>>(smds: &Vec<PathBuf>, qc_file: P, output_dir: P) -> Result<()> {
    let mut qc_path_parent = qc_file.as_ref().to_path_buf().clone();
    qc_path_parent.pop();
    let x = smds
        .iter()
        .map(|smd| PathBuf::from(&qc_path_parent.join(smd)))
        .collect::<Vec<PathBuf>>();
    let y = smds
        .iter()
        .map(|smd| PathBuf::from(output_dir.as_ref().join(smd)))
        .collect::<Vec<PathBuf>>();

    for smd in x.into_iter().zip(y) {
        let mut smd_parent = smd.1.to_owned();
        smd_parent.pop();
        fs::create_dir_all(&smd_parent)
            .with_context(|| format!("Failed to create {}", smd.1.display()))?;
        fs::copy(&smd.0, &smd.1).with_context(|| {
            format!("Failed to copy {} to {}", smd.0.display(), smd.1.display())
        })?;
    }
    Ok(())
}
