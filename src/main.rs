mod model;

use clap::{Parser, ValueEnum};

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

fn main() {
    let args = Args::parse();
}
