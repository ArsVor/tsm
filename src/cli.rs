use std::path::Path;

use clap::{ArgGroup, Parser};

use crate::err_exit;

/// Tmux Session Manager simple program for manage the TMUX sessions
/// (automatically or manually load with presets)
#[derive(Parser, Debug)]
#[command(version, 
    about, 
    long_about = None, 
    group(
        ArgGroup::new("mode")
            .required(true)
            .args(["check", "sync", "remove", "query", "upload"]),
    ),
    group(
        ArgGroup::new("switch")
        .required(false)
        .args(["autoload", "disable_autoload"])
    ),
    group(
        ArgGroup::new("schemas")
        .required(false)
        .args(["project", "schema"])
    ),
    )]
pub struct Cli {
    /// Check helth
    #[arg(short = 'C')]
    pub check: bool,

    /// Query
    #[arg(short = 'Q')]
    pub query: bool,

    /// Remove
    #[arg(short = 'R')]
    pub remove: bool,

    /// Synchronize
    #[arg(short = 'S')]
    pub sync: bool,

    /// Upload session
    #[arg(short = 'U')]
    pub upload: bool,

    /// alter target (do actoion with template) 
    #[arg(
        short = 'a',
        conflicts_with_all = ["upload"]
    )]
    pub alter_target: bool,

    /// path
    #[arg(
        short = 'c',
        conflicts_with_all = ["check", "remove", "upload"]
    )]
    pub path: Option<String>,

    /// disable autoload
    #[arg(
        short = 'd',
        requires = "sync",
        conflicts_with_all = ["remove", "upload"]
    )]
    pub disable_autoload: bool,

    /// view info
    #[arg(
        short = 'i',
        requires = "query",
        requires = "name",
        conflicts_with_all = ["check", "remove", "sync", "upload"]
    )]
    pub info: bool,

    /// start session with schema from file
    #[arg(
        short = 'f',
        value_name = "FILE",
        conflicts_with_all = ["check", "query", "remove", "sync"]
    )]
    pub schema_file: Option<String>,

    /// no attach
    #[arg(
        short = 'n',
        conflicts_with_all = ["check", "query", "remove", "sync"]
    )]
    pub no_attach: bool,

    /// alias for '-s project'
    #[arg(
        short = 'p',
        conflicts_with_all = ["check", "remove", "upload"]
    )]
    pub project: bool,

    /// schema (default = simple)
    #[arg(
        short = 's',
        conflicts_with_all = ["check", "query", "remove", "alter_target"]
    )]
    pub schema: Option<String>,

    /// tabled output
    #[arg(
        short = 't',
        conflicts_with_all = ["check", "remove", "sync", "upload"]
    )]
    pub tabled: bool,

    /// autoload
    #[arg(
        short = 'u',
        conflicts_with_all = ["remove"]
    )]
    pub autoload: bool,

    /// update
    #[arg(
        short = 'y',
        requires = "sync",
        conflicts_with_all = ["check", "query", "remove", "upload"]
    )]
    pub update: bool,

    /// input new name
    #[arg(
        short,
        value_name = "NEW-NAME",
        requires = "update",
        conflicts_with_all = ["check", "query", "remove", "upload"]
    )]
    pub rename: Option<String>,

    /// name
    #[arg(
        required_unless_present_any(["check", "query"]),
        required_unless_present_all = ["autoload", "upload"]
    )]
    pub name: Option<String>,
}

impl Cli {
    pub fn effective_schema(&self) -> &str {
         if let Some(schema) = &self.schema {
            schema
         } else if self.project {
            "project"
        } else {
            "simple"
         }
    }

    pub fn get_command(&self) -> &str {
        if self.check {
            "check"
        } else if self.query {
            "query"
        } else if self.remove {
            "remove"
        } else if self.sync {
            "sync"
        } else if self.upload {
            "upload"
        } else {
            unreachable!()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap()
    }

    pub fn get_path(&self) -> String {
        let raw_path = if let Some(cli_path) = &self.path {
            cli_path.clone()
        } else {
            ".".into()
        };

        let path = Path::new(&raw_path);

        if !path.exists() {
            err_exit!(format!("path: '{}' does not exist", raw_path.yellow()));
        }

        path.canonicalize()
            .unwrap_or_else(|e| {err_exit!(format!("cannot canonicalize path: '{}'", e.yellow()));})
            .to_string_lossy()
            .into_owned()
    }
}
