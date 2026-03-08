pub mod cli;
pub mod db;
pub mod handlers;
pub mod makros;
pub mod init;

use crate::cli::Cli;
use clap::Parser;
use db::schema::open_connection_with_fk;
use init::init_paths;

fn main() {
    let cli = Cli::parse();
    // let conn = open_connection_with_fk("/home/ars/projects/tsm/tsm.db").unwrap();

    let paths = init_paths();

    let config_file = paths.config_dir.join("config.toml");

    if !config_file.exists() {
        std::fs::write(&config_file, "database = \"tsm.db\"").unwrap();
    }

    let conn = open_connection_with_fk(paths.db.to_str().unwrap()).unwrap();

    let result = match cli.get_command() {
        "check" => handlers::check::route(&conn, cli),
        "query" => handlers::query::route(&conn, cli),
        "remove" => handlers::remove::route(&conn, cli),
        "sync" => handlers::sync::route(&conn, cli),
        "upload" => handlers::upload::route(&conn, cli),
        _ => Ok(()),
    };

    if let Err(e) = result {
        println!("OoPS!");
        err_exit!(&e);
    }
}
