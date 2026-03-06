use std::process::Command;

use rusqlite::{Connection, Result};

use crate::{
    cli::Cli,
    db::{models::SessionInfo, sql::{self, Queries, Query}},
};

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    upload(conn, cli)?;

    Ok(())
}

fn upload(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if cli.autoload {
        queries.push(Query::new("is_autoloaded", true));
    } else {
        queries.push(Query::new("nmae", cli.get_name()));
    }

    let sessions: Vec<SessionInfo> = sql::get::session_info(conn, &queries)?;

    if !sessions.is_empty() {
        let attach: &str = if cli.no_attach {
            "--no_attach"
        } else {
            "--attach"
        };

        for session in sessions {
            let command_str: String = format!("{} {} {} {}", &session.template_path, attach, &session.name, &session.path);

            let _ = Command::new("bash")
                .arg("-c")
                .arg(command_str)
                .output()
                .expect("Failed to execute command");
        }
    }

    Ok(())
}
