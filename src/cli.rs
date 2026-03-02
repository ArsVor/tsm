use clap::{ArgGroup, Parser};

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
        .args(["enable", "disable"])
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

    /// Synchronize
    #[arg(short = 'S')]
    pub sync: bool,

    /// Remove
    #[arg(short = 'R')]
    pub remove: bool,

    /// Query
    #[arg(short = 'Q')]
    pub query: bool,

    /// Upload session
    #[arg(short = 'U')]
    pub upload: bool,

    /// disable autoload
    #[arg(
        short = 'd',
        requires = "sync",
        conflicts_with_all = ["remove", "upload"]
    )]
    pub disable: bool,

    /// view info
    #[arg(
        short = 'i',
        requires = "query",
        requires = "name",
        conflicts_with_all = ["check", "sync", "remove", "upload"]
    )]
    pub info: bool,

    /// alias for '-s project'
    #[arg(
        short = 'p',
        conflicts_with_all = ["check", "remove", "upload"]
    )]
    pub project: bool,

    /// schema (default = simple)
    #[arg(
        short = 's',
        conflicts_with_all = ["check", "remove", "upload"]
    )]
    pub schema: Option<String>,

    /// do actoion with template 
    #[arg(short = 't')]
    pub template: bool,

    /// enable autoload
    #[arg(
        short = 'u',
        requires = "sync",
        conflicts_with_all = ["query", "remove"]
    )]
    pub enable: bool,

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

    /// path
    #[arg(
        short = 'c',
        conflicts_with_all = ["check", "remove", "upload"]
    )]
    pub path: Option<String>,

    /// name
    #[arg(
        required_unless_present_any(["check", "query"]),
        required_unless_present_all = ["enable", "upload"]
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
}
