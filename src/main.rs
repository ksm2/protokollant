#[macro_use]
extern crate pest_derive;

mod model;
mod parser;

use crate::parser::parse_file;
use clap::{Parser, ValueEnum};
use std::io::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    change: Change,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
enum Change {
    Major,
    Minor,
    Patch,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let changelog = parse_file("CHANGELOG.md")?;
    println!("{changelog:#?}");

    Ok(())
}
