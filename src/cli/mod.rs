use crate::qc::QC;
use anyhow::{bail, Result};
use comfy_table::{Cell, Color, Row, Table};
use console::{style, Term};
use globber::Pattern;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "qcmerger")]
pub struct Opt {
    /// output path
    #[structopt(
        value_name("PATH"),
        short,
        long,
        parse(from_os_str),
        default_value = "output/"
    )]
    pub output: PathBuf,
    /// qc file that has animations you want to use in the base file
    #[structopt(value_name("QC FILE"), short, long, parse(from_os_str))]
    pub file: PathBuf,
    /// base qc file that has animations that will be replaced/added
    #[structopt(value_name("QC FILE"), short, long, parse(from_os_str))]
    pub base_file: PathBuf,
    /// don't clear the console
    #[structopt(short, long)]
    pub dont_clear: bool,
}

pub struct SequenceTable(Table);

impl SequenceTable {
    // creates a new SequenceTable from a QC struct
    pub fn from(qc: &QC) -> Self {
        let mut table = Table::new();
        table.set_header(vec!["available animations", "  ", "selected animations"]);
        table.load_preset("││──╞═╪╡│    ┬┴┌┐└┘");

        let mut sorted_names = qc
            .sequences
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<&str>>();
        sorted_names.sort();

        for name in sorted_names {
            table.add_row(vec![
                name.to_owned(),
                "  ".to_owned(),
                String::from_utf8(vec![b' '; name.len()]).unwrap(),
            ]);
        }
        SequenceTable(table)
    }

    // if sequence is already get_selected, deselect it, otherwise select it
    pub fn toggle_select<T: AsRef<str>>(&mut self, name: T) {
        for row in self.0.row_iter_mut() {
            let cells = row.cell_iter();

            let left_colum = cells.clone().next().unwrap().get_content();
            let middle_colum = cells.clone().nth(1).unwrap().get_content();
            // no arrow == not selected, arrow == selected
            if middle_colum == "  ".to_owned() && left_colum == name.as_ref().to_owned() {
                *row = Row::from(vec![
                    Cell::new(name.as_ref()),
                    Cell::new("->").fg(Color::Green),
                    Cell::new(name.as_ref()).fg(Color::Green),
                ]);
            } else if middle_colum == "->".to_owned() && left_colum == name.as_ref().to_owned() {
                *row = Row::from(vec![
                    name.as_ref().to_owned(),
                    "  ".to_owned(),
                    "  ".to_owned(),
                ]);
            }
        }
    }

    // deselects everything
    pub fn clear_selection(&mut self) {
        for row in self.0.row_iter_mut() {
            let mut cells = row.cell_iter();
            if cells.clone().nth(1).unwrap().get_content() == "->".to_owned() {
                *row = Row::from(vec![
                    cells.next().unwrap().get_content(),
                    "  ".to_owned(),
                    "  ".to_owned(),
                ]);
            }
        }
    }

    // prints the table, input message, and possible errors
    pub fn print_table(&self, term: &Term, error: &mut String, clear_console: bool) -> Result<()> {
        if !clear_console {
            term.clear_screen()?;
        }
        println!("{}", self.0);
        println!(
            "Enter names separated by a space (you can use globs)
Enter nothing to confirm, enter {} to exit, {} to clear
{}",
            style("exit").bold(),
            style("clear").bold(),
            error,
        );
        error.clear();
        Ok(())
    }

    // noinspection RsSelfConvention <- ignore this
    pub fn get_selected(&mut self) -> Vec<String> {
        self.0
            .row_iter()
            .map(|row| row.cell_iter())
            .filter(|cell| cell.clone().nth(1).unwrap().get_content() == "->".to_owned())
            .map(|cell| cell.clone().next().unwrap().get_content())
            .collect()
    }
}

pub fn process_input<T: AsRef<str>>(qc: &QC, input: T) -> Result<Vec<String>> {
    let mut sequences = input
        .as_ref()
        .split_whitespace()
        .map(|name| name.to_owned())
        .collect::<Vec<String>>();

    let all_sequences = qc
        .sequences
        .iter()
        .map(|(name, _)| name.to_owned())
        .collect::<Vec<String>>();

    let prefixes = sequences
        .iter()
        .filter(|name| name.contains('*'))
        .map(|name| name.to_owned())
        .collect::<Vec<String>>();

    for prefix in prefixes {
        let pattern = Pattern::new(&prefix)?;
        for name in all_sequences.iter() {
            if pattern.matches(&name) {
                sequences.push(name.to_owned());
            }
        }
    }

    sequences.retain(|name| !name.contains('*'));

    // checks if a key exists
    if sequences
        .iter()
        .any(|name| !qc.sequences.contains_key(name.as_str()))
    {
        bail!(
            "Failed to find: {}",
            sequences
                .iter()
                .filter(|&name| !qc.sequences.contains_key(name))
                .map(|name| style(name).red().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    Ok(sequences)
}
