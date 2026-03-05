pub mod cli;
pub mod db;
pub mod handlers;
pub mod makros;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    // println!("SCHEMA: {}", &args.effective_schema());
    // println!("Args: {:?}", &args);
    // println!("Path: {}", &args.get_path());
}
