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
use crate::model::{Change, Release};
use crate::parser::parse_str;
use clap::Parser;
use std::fs::{read_to_string, write};
use std::io::{stderr, stdout, Result, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    change: Change,

    #[arg(long, help = "Print all changes to stdout and exit")]
    diff: bool,

    #[arg(long, help = "Create an unreleased section")]
    unreleased: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let changelog_str = read_to_string("CHANGELOG.md")?;
    let mut changelog = parse_str(&changelog_str);

    if args.unreleased && !changelog.has_unreleased() {
        changelog.releases.insert(0, Release::default());
    }

    let mut diffs = Vec::<FileDiff>::new();
    let bumped = changelog.bump(args.change);
    if !bumped {
        eprintln!("No changes to release");
    } else {
        let new_version = changelog.version().unwrap();
        eprintln!("Releasing new version {}", new_version);

        let manifest_types = detect_manifests()?;
        for manifest_type in manifest_types {
            eprintln!("Detected {}", manifest_type);
            let manifest_diffs = manifest_type.change_version(&new_version, !args.diff)?;
            for diff in manifest_diffs {
                diffs.push(diff);
            }
        }
    }

    let new_str = generate_str(&changelog);

    let file_diff = FileDiff::new("CHANGELOG.md", changelog_str, new_str.clone());
    diffs.push(file_diff);

    let mut writer: Box<dyn Write> = if args.diff {
        Box::new(stdout())
    } else {
        Box::new(stderr())
    };
    diff_files(&mut writer, &diffs)?;

    if bumped && !args.diff {
        write("CHANGELOG.md", &new_str)?;

        let new_version = changelog.version().unwrap();
        println!("v{}", new_version);
    }

    if !bumped {
        std::process::exit(1);
    }

    Ok(())
}
