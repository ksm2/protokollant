#[macro_use]
extern crate pest_derive;

mod diff;
mod generate;
mod manifests;
mod model;
mod parser;

use crate::diff::{diff_files, FileDiff};
use crate::generate::generate_str;
use crate::manifests::detect_manifests;
use crate::model::Change;
use crate::parser::parse_str;
use clap::Parser;
use std::fs::{read_to_string, write};
use std::io::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    change: Change,

    #[arg(long)]
    diff: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let changelog_str = read_to_string("CHANGELOG.md")?;
    let mut changelog = parse_str(&changelog_str);

    let mut diffs = Vec::new();
    let bumped = changelog.bump(args.change);
    if !bumped {
        eprintln!("No changes to release");
    } else {
        let new_version = changelog.version().unwrap();

        let manifest_types = detect_manifests()?;
        for manifest_type in manifest_types {
            let diff = manifest_type.change_version(&new_version)?;
            diffs.push(diff);
        }
    }

    let new_str = generate_str(&changelog);

    if args.diff {
        let file_diff = FileDiff::new("CHANGELOG.md", changelog_str, new_str);
        diffs.push(file_diff);

        diff_files(&diffs);
    } else if bumped {
        write("CHANGELOG.md", &new_str)?;
    }

    if !bumped {
        std::process::exit(1);
    }

    Ok(())
}
