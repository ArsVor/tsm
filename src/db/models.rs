use rusqlite::{Result, Row};
use tabled::Tabled;

#[derive(Debug, Clone, Tabled)]
pub struct Template {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Path")]
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: i32,
    pub is_autoloaded: bool,
    pub name: String,
    pub path: String,
    pub template_id: i32,
}

#[derive(Debug, Clone, Tabled)]
pub struct SessionInfo {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Autoload")]
    pub is_autoloaded: String,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Schema")]
    pub template: String,
}

impl Template {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
        })
    }
}

impl Session {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
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
            is_autoloaded: row.get("is_autoloaded")?,
            name: row.get("name")?,
            path: row.get("path")?,
            template: row.get("template_name")?,
        })
    }
}
