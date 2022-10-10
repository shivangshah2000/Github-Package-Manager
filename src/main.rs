mod cli;
mod release_api;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
