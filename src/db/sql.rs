use crate::db::models::{Session, Template};
use rusqlite::{Connection, Result, ToSql, params, params_from_iter, types::Value};

#[derive(Debug, Clone)]
pub struct Query {
    name: String,
    value: Value,
}

#[derive(Debug, Clone)]
pub struct Queries {
    qvec: Vec<Query>,
}

impl Query {
    pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Queries {
    pub fn new() -> Self {
        Self { qvec: Vec::new() }
    }

    pub fn push(&mut self, q: Query) {
        self.qvec.push(q);
    }

    /// SET id = ?, name = ?, ... path = ?
    pub fn get_update_set(&self) -> String {
        let where_clause = self
            .qvec
            .iter()
            .map(|q| format!("{} = ?", q.name()))
            .collect::<Vec<_>>()
            .join(", ");

        format!(" SET {}", &where_clause)
    }

    /// WHERE id = ? AND name = ? ...
    pub fn get_where(&self) -> String {
        let where_clause = self
            .qvec
            .iter()
            .map(|q| format!("{} = ?", q.name()))
            .collect::<Vec<_>>()
            .join(" AND ");

        format!(" WHERE {}", &where_clause)
    }

    pub fn get_dyn_params(&self) -> Vec<&dyn ToSql> {
        self.qvec.iter().map(|q| &q.value as &dyn ToSql).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.qvec.is_empty()
    }
}

impl Default for Queries {
    fn default() -> Self {
        Self::new()
    }
}

pub mod create {

    use super::*;

    pub fn session(conn: &Connection, session: Session) -> Result<()> {
        conn.execute(
            "INSERT INTO session
            (is_autoloaded, name, path, template_id) VALUES (?1, ?2, ?3, ?4)",
            params![
                session.is_autoloaded,
                session.name,
                session.path,
                session.template_id
            ],
        )?;
        Ok(())
    }

    pub fn template(conn: &Connection, template: Template) -> Result<()> {
        conn.execute(
            "INSERT INTO template
            (name, path) VALUES (?1, ?2)",
            params![template.name, template.path,],
        )?;
        Ok(())
    }
}

pub fn is_instance(conn: &Connection, table: &str, name: &str) -> Result<bool> {
    let sql: String = format!("SELECT EXISTS(SELECT 1 FROM {table} WHERE name = ?)");

    conn.query_row(&sql, params![name], |row| row.get(0))
}

pub mod get {

    use rusqlite::OptionalExtension;

    use crate::{db::models::SessionInfo, err_exit};

    use super::*;

    pub fn session(conn: &Connection, queries: &Queries) -> Result<Vec<Session>> {
        let mut sql = "SELECT * FROM session".to_string();

        if !queries.is_empty() {
            sql.push_str(&queries.get_where());
        }

        let sessions: Vec<Session> = conn
            .prepare(&sql)?
            .query_map(
                params_from_iter(queries.get_dyn_params()),
                Session::from_row,
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    pub fn session_info(conn: &Connection, queries: &Queries) -> Result<Vec<SessionInfo>> {
        let mut sql = "
            SELECT 
                s.id as id,
                s.is_autoloaded as is_autoloaded,
                s.name as name,
                s.path as path,
                t.name as template_name,
                t.parh as template_path
            FROM session s
            JOIN template t ON t.id = s.template_id
        "
        .to_string();

        if !queries.is_empty() {
            sql.push_str(&queries.get_where());
        }

        let sessions: Vec<SessionInfo> = conn
            .prepare(&sql)?
            .query_map(
                params_from_iter(queries.get_dyn_params()),
                SessionInfo::from_row,
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    pub fn template(conn: &Connection, queries: &Queries) -> Result<Vec<Template>> {
        let mut sql = "SELECT * FROM template".to_string();

        if !queries.is_empty() {
            sql.push_str(&queries.get_where());
        }

        let templates: Vec<Template> = conn
            .prepare(&sql)?
            .query_map(
                params_from_iter(queries.get_dyn_params()),
                Template::from_row,
            )?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(templates)
    }

    pub fn template_id_or_err(conn: &Connection, name: &str) -> Result<i32> {
        conn.query_row(
            "SELECT id FROM template WHERE name = ?",
            params![name],
            |row| row.get(0),
        )
        .optional()?
        .ok_or_else(|| {
            err_exit!(format!("schema: '{}', does not exist", &name.yellow()));
        })
    }
}

pub fn remove(conn: &Connection, table: &str, name: &str) -> Result<()> {
    let sql = format!("DELETE FROM {} WHERE name = ?", &table);
    conn.execute(&sql, params![name])?;
    Ok(())
}

pub fn update(conn: &Connection, queries: &Queries, table: &str, name: &str) -> Result<()> {
    let mut dyn_params: Vec<&dyn ToSql> = queries.get_dyn_params();
    dyn_params.push(&name);

    let sql = format!(
        "UPDATE {} {} WHERE name = ?",
        &table,
        &queries.get_update_set()
    );

    conn.execute(&sql, params_from_iter(dyn_params))?;

    Ok(())
}
