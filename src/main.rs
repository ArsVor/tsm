pub mod cli;
pub mod db;
pub mod handlers;
pub mod makros;

use crate::cli::Cli;
use clap::Parser;
use db::schema::open_connection_with_fk;
use rusqlite::Connection;

fn main() {
    let cli = Cli::parse();
    let conn: Connection = open_connection_with_fk("./tsm.db").unwrap();
    // println!("SCHEMA: {}", &args.effective_schema());
    // println!("Args: {:?}", &args);
    // println!("Path: {}", &args.get_path());

    let result = match cli.get_command() {
        "check" => handlers::check::route(&conn, cli),
        "query" => handlers::query::route(&conn, cli),
        "remove" => handlers::remove::route(&conn, cli),
        "sync" => handlers::sync::route(&conn, cli),
        "upload" => handlers::upload::route(&conn, cli),
        _ => Ok(())
    };

    if let Err(e) = result {
        println!("OoPS!");
        err_exit!(&e);
    }
}
