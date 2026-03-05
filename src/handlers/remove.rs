use rusqlite::{Connection, Result};

use crate::{cli::Cli, db::sql, warn_exit};


pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    if cli.alter_target {
        remove_template(conn, cli)?;
    } else {
        remove_session(conn, cli)?;
    }
    Ok(())
}

fn remove_session(conn: &Connection, cli: Cli) -> Result<()> {
    let name: String = cli.get_name();

    if !sql::is_instance(conn, "session", &name)? {
        warn_exit!(format!("session '{}' does not exist.", name.yellow()));
    }

    sql::remove(conn, "session", &name)?;

    Ok(())
}

fn remove_template(conn: &Connection, cli: Cli) -> Result<()> {
    let name: String = cli.get_name();

    if !sql::is_instance(conn, "template", &name)? {
        warn_exit!(format!("schema '{}' does not exist.", name.yellow()));
    }

    sql::remove(conn, "template", &name)?;

    Ok(())
}
