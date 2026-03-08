use rusqlite::{Result, Row};
use tabled::Tabled;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: i32,
    pub added: String,
    pub edited: String,
    pub is_autoloaded: bool,
    pub name: String,
    pub path: String,
    pub template_id: i32,
}

#[derive(Debug, Clone, Tabled)]
pub struct SessionInfo {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(skip)]
    pub added: String,
    #[tabled(skip)]
    pub edited: String,
    #[tabled(rename = "Autoload")]
    pub is_autoloaded: bool,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Schema")]
    pub template_name: String,
    #[tabled(skip)]
    pub template_path: String,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub id: i32,
    pub added: String,
    pub edited: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Tabled)]
pub struct TemplateInfo {
    #[tabled(skip)]
    pub id: i32,
    #[tabled(skip)]
    pub added: String,
    #[tabled(skip)]
    pub edited: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Session count")]
    pub session_count: i32,
}

impl Session {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            added: row.get("added")?,
            edited: row.get("edited")?,
            is_autoloaded: row.get("is_autoloaded")?,
            name: row.get("name")?,
            path: row.get("path")?,
            template_id: row.get("template_id")?,
        })
    }
}

impl SessionInfo {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            added: row.get("added")?,
            edited: row.get("edited")?,
            is_autoloaded: row.get("is_autoloaded")?,
            name: row.get("name")?,
            path: row.get("path")?,
            template_name: row.get("template_name")?,
            template_path: row.get("template_path")?,
        })
    }
}

impl Template {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            added: row.get("added")?,
            edited: row.get("edited")?,
            name: row.get("name")?,
            path: row.get("path")?,
        })
    }
}

impl TemplateInfo {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            added: row.get("added")?,
            edited: row.get("edited")?,
            name: row.get("name")?,
            path: row.get("path")?,
            session_count: row.get("session_count")?,
        })
    }
}
