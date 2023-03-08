#[macro_use]
extern crate pest_derive;

mod diff;
mod generate;
mod model;
mod parser;

use crate::diff::diff_changelogs;
use crate::generate::generate_str;
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

    changelog.bump(args.change);

    let new_str = generate_str(&changelog);

    if args.diff {
        let changes = diff_changelogs("CHANGELOG.md", &changelog_str, &new_str);
        std::process::exit(if changes { 1 } else { 0 });
    } else {
        write("CHANGELOG.md", &new_str)?;
    }

    Ok(())
}
