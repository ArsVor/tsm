use owo_colors::OwoColorize;
use std::path::Path;

use rusqlite::{Connection, Result};

use crate::{cli::Cli, db::{models::{SessionInfo, TemplateInfo}, sql::{self, Queries, Query}}, warn_exit};

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    if cli.alter_target {
        check_template(conn, cli)?;
    } else {
        check_session(conn, cli)?;
    }

    Ok(())
}

fn check_session(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if cli.name.is_some() {
        queries.push(Query::new("s.name", cli.get_name()));
    }

    let sessions: Vec<SessionInfo> = sql::get::session_info(conn, &queries)?;

    if sessions.is_empty() {
        warn_exit!(format!("session '{}' does not exist", cli.get_name().yellow()));
    }

    for session in sessions {
        let path_ok = Path::new(&session.path).is_dir();
        let template_ok = Path::new(&session.template_path).is_file();

        if path_ok && template_ok {
            ok("Session", &session.name);
        } else {
            println!("{:<8}: '{}'", "Session", &session.name.blue());

            if !path_ok {
                fail("path  ", &session.path);
            }

            if !template_ok {
                fail("schema", &session.template_name);
            }
        }
    }
    Ok(())
}

fn check_template(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if cli.name.is_some() {
        queries.push(Query::new("t.name", cli.get_name()));
    }

    let templates: Vec<TemplateInfo> = sql::get::template_info(conn, &queries)?;

    if templates.is_empty() {
        warn_exit!(format!("schema '{}' does not exist", cli.get_name().yellow()));
    }

    for template in templates {
        match Path::new(&template.path).is_file() {
            true => ok("Schema", &template.name),
            false => {
                println!("{:<8}: '{}'", "Schema", &template.name.blue());
                fail("path", &template.path)
            },
        }
    }

    Ok(())
}

fn fail(title: &str, item: &str) {
    println!("{:>8}: {} '{}'", &title, "✗ fail".red(), item.yellow());
}

fn ok(title: &str, item: &str) {
    println!("{:<8}: '{}' {}", &title, item.blue(), "✓ ok".green());
}
