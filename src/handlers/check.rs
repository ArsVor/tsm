// Path::new(cli_path).exists()

use rusqlite::{Connection, Result};

use crate::cli::Cli;

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    Ok(())
}
