use owo_colors::OwoColorize;
use rusqlite::{Connection, Result};
use tabled::{Table, settings::Style};

use crate::{
    cli::Cli,
    db::{
        models::{SessionInfo, TemplateInfo},
        sql::{self, Queries, Query},
    },
};

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    if cli.info {
        query_one(conn, cli)?;
    } else if cli.alter_target {
        query_template(conn, cli)?;
    } else {
        query_session(conn, cli)?;
    }
    Ok(())
}

fn query_one(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    queries.push(Query::new(
        if cli.alter_target { "t.name" } else { "s.name" },
        cli.get_name(),
    ));

    if cli.alter_target {
        let query_set: Vec<TemplateInfo> = sql::get::template_info(conn, &queries)?;

        if !query_set.is_empty() {
            let template: &TemplateInfo = &query_set[0];

            println!("{:<17} : {}", "Name", &template.name);
            println!("{:<17} : {}", "Added", &template.added);
            println!("{:<17} : {}", "Edited", &template.edited);
            println!("{:<17} : '{}'", "Path", &template.path.yellow());
            println!("{:<17} : {}", "Attached sessions", &template.session_count);
        }
    } else {
        let query_set: Vec<SessionInfo> = sql::get::session_info(conn, &queries)?;

        if !query_set.is_empty() {
            let session: &SessionInfo = &query_set[0];

            println!("{:<11} : {}", "Name", &session.name);
            println!("{:<11} : {}", "Added", &session.added);
            println!("{:<11} : {}", "Edited", &session.edited);
            println!(
                "{:<11} : {}",
                "Autoloaded",
                if session.is_autoloaded { "yes" } else { "no" }
            );
            println!("{:<11} : '{}'", "Path", &session.path.yellow());
            println!("{:<11} : {}", "Schema", &session.template_name);
        }
    }

    Ok(())
}

fn query_session(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if let Some(path) = cli.path {
        queries.push(Query::new("s.path", path));
    }

    if let Some(name) = cli.name {
        queries.push(Query::new("s.name", name));
    }

    if cli.autoload || cli.disable_autoload {
        queries.push(Query::new("s.is_autoloaded", cli.autoload));
    }

    if let Some(schema) = cli.schema {
        let template_id: i32 = sql::get::template_id_or_err(conn, &schema)?;
        queries.push(Query::new("s.template_id", template_id));
    }

    let query_set: Vec<SessionInfo> = sql::get::session_info(conn, &queries)?;

    if cli.tabled {
        let mut table: Table = Table::new(query_set);
        table.with(Style::rounded());
        println!("{}", &table);
    } else {
        for session in query_set {
            println!("{}", session.name);
        }
    }

    Ok(())
}

fn query_template(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if let Some(path) = cli.path {
        queries.push(Query::new("t.path", path));
    }

    if let Some(name) = cli.name {
        queries.push(Query::new("t.name", name));
    }

    let query_set: Vec<TemplateInfo> = sql::get::template_info(conn, &queries)?;

    if cli.tabled {
        let mut table: Table = Table::new(query_set);
        table.with(Style::rounded());
        println!("{}", &table);
    } else {
        for template in query_set {
            println!("{}", template.name);
        }
    }

    Ok(())
}
