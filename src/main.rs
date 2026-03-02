pub mod cli;
pub mod db;
pub mod handlers;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    println!("SCHEMA: {}", &args.effective_schema());
    println!("Args: {:?}", &args);
}
