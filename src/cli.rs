#[cfg(feature = "argh")]
use argh::FromArgs;
#[cfg(feature = "clap")]
use clap::{Parser, Subcommand};

#[cfg(feature = "clap")]
#[derive(Parser, Debug)]
#[command(about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[cfg(feature = "clap")]
#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Query dependencies of packages
    #[command(visible_alias = "dep")]
    Depends { packages: Vec<String> },
    /// Query reverse dependencies of packages
    #[command(visible_alias = "rdep")]
    Rdepends { packages: Vec<String> },
    /// Get package information
    #[command(visible_alias = "info")]
    Show { packages: Vec<String> },
    /// Search for packages
    Search {
        pattern: String,
        #[arg(long, short, visible_alias = "no-redir")]
        search_only: bool,
    },
    /// List 100 latest source updates
    Updates,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// Get package information from packages.aosc.io
pub struct Cli {
    #[argh(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommands {
    Depends(Depends),
    Rdepends(Rdepends),
    Show(Show),
    Search(Search),
    Updates(Updates),
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// Query dependencies of packages
#[argh(subcommand, name = "depends")]
pub struct Depends {
    #[argh(positional, greedy)]
    pub packages: Vec<String>,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// Query reverse dependencies of packages
#[argh(subcommand, name = "rdepends")]
pub struct Rdepends {
    #[argh(positional, greedy)]
    pub packages: Vec<String>,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// Get package information
#[argh(subcommand, name = "show")]
pub struct Show {
    #[argh(positional, greedy)]
    pub packages: Vec<String>,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// Search for packages
#[argh(subcommand, name = "search")]
pub struct Search {
    #[argh(positional)]
    pub pattern: String,
    #[argh(switch, short = 's')]
    /// search without redirecting to the exact match
    pub search_only: bool,
}

#[cfg(feature = "argh")]
#[derive(FromArgs, Debug)]
/// List 100 latest source updates
#[argh(subcommand, name = "updates")]
pub struct Updates {}
