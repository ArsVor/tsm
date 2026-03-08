use std::process::{Command, Stdio};

use rusqlite::{Connection, Result};

use crate::{
    cli::Cli,
    db::{
        models::SessionInfo,
        sql::{self, Queries, Query},
    },
};

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    upload(conn, cli)?;

    Ok(())
}

fn upload(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if cli.autoload {
        queries.push(Query::new("s.is_autoloaded", true));
    } else {
        queries.push(Query::new("s.name", cli.get_name()));
    }

    let sessions: Vec<SessionInfo> = sql::get::session_info(conn, &queries)?;

    if !sessions.is_empty() {
        let attach: &str = if cli.no_attach {
            "--no_attach"
        } else {
            "--attach"
        };

        for session in sessions {
            Command::new(&session.template_path)
                .args([attach, &session.name, &session.path])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap();
            //     let command_str: String = format!(
            //         "{} {} {} {}",
            //         &session.template_path, attach, &session.name, &session.path
            //     );
            //
            //     let _ = Command::new("bash")
            //         .arg("-c")
            //         .arg(command_str)
            //         .output()
            //         .expect("Failed to execute command");
        }
    }

    Ok(())
}
