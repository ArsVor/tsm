use rusqlite::{Connection, Result};

use crate::{
    cli::Cli,
    db::{
        models::{Session, Template},
        sql::{self, Queries, Query},
    },
};

pub fn route(conn: &Connection, cli: Cli) -> Result<()> {
    if cli.update {
        update(conn, cli)?;
    } else if cli.template {
        create_template(conn, cli)?;
    } else {
        create_session(conn, cli)?;
    }
    Ok(())
}

fn create_session(conn: &Connection, cli: Cli) -> Result<()> {
    let templ_id: i32 = sql::get::template_id_or_err(conn, &cli.schema.clone().unwrap())?;

    let session: Session = Session {
        id: 0,
        is_autoloaded: cli.autoload,
        name: cli.get_name(),
        path: cli.get_path(),
        template_id: templ_id,
    };

    sql::create::session(conn, session)?;

    Ok(())
}

fn create_template(conn: &Connection, cli: Cli) -> Result<()> {
    let template: Template = Template {
        id: 0,
        name: cli.get_name(),
        path: cli.get_path(),
    };

    sql::create::template(conn, template)?;

    Ok(())
}

fn update(conn: &Connection, cli: Cli) -> Result<()> {
    let mut queries: Queries = Queries::new();

    if let Some(name) = &cli.rename {
        queries.push(Query::new("name", name.clone()));
    }

    if cli.path.is_some() {
        queries.push(Query::new("path", cli.get_path()));
    }

    let table = if cli.template { "template" } else { "session" };

    if !cli.template {
        if cli.autoload || cli.disable_autoload {
            queries.push(Query::new("is_autoloaded", cli.autoload));
        };

        let template_name = cli
            .schema
            .clone()
            .or_else(|| cli.project.then(|| "project".to_string()));

        if let Some(name) = template_name {
            let id = sql::get::template_id_or_err(conn, &name)?;
            queries.push(Query::new("template_id", id));
        }
    };

    sql::update(conn, &queries, table, &cli.get_name())?;


    Ok(())
}
