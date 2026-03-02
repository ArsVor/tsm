use rusqlite::{Connection, Result};

pub fn init_shema(conn: &Connection) -> Result<()> {
    create_session_table(conn)?;
    create_template_table(conn)?;
    Ok(())
}

fn create_session_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXIST session
        id                  INTEGER PRIMARY KEY,
        is_autoloaded       INTEGER DEFAULT 0,
        name                TEXT NOT NULL UNIQUE,
        path                TEXT NOT NULL,
        template_id         INTEGER NOT NULL,
        FOREIGN KEY(template_id) REFERENCES template(id) ON DELETE RESTRICT",
        [],
    )?;
    Ok(())
}

fn create_template_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXIST template
        id      INTEGER PRIMARY KEY,
        name    TEXT NOT NULL UNIQUE,
        path    TEXT NOT NULL UNIQUE",
        [],
    )?;
    Ok(())
}
