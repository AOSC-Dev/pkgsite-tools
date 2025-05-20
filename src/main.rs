use anyhow::Result;
#[cfg(feature = "clap")]
use clap::Parser;
use pkgsite_lib::PackagesSiteClient;

mod cli;
mod views;

use cli::*;
use pkgsite_tools::{dedup_packages, print_res};

async fn run() -> Result<()> {
    #[cfg(feature = "clap")]
    let args = Cli::parse();

    #[cfg(feature = "argh")]
    let args = argh::from_env::<Cli>();

    #[cfg(feature = "reqwest")]
    let pkgsite = PackagesSiteClient::default_url()?;

    #[cfg(feature = "nyquest")]
    nyquest_preset::register();
    #[cfg(feature = "nyquest")]
    let pkgsite = PackagesSiteClient::default_url().await?;

    #[cfg(feature = "clap")]
    match args.subcommands {
        Some(cmd) => match cmd {
            Subcommands::Depends { packages } => {
                print_res!(annotated pkgsite, depends, views::depends::DependsView, packages);
            }
            Subcommands::Rdepends { packages } => {
                print_res!(annotated pkgsite, rdepends, views::rdepends::RDependsView, packages);
            }
            Subcommands::Show { packages } => {
                print_res!(unannotated pkgsite, info, views::info::InfoView, packages);
            }
            Subcommands::Search {
                pattern,
                search_only,
            } => {
                print_res!(single pkgsite, search, views::search::SearchView, &pattern, search_only);
            }
            Subcommands::Updates => {
                print_res!(single pkgsite, updates, views::updates::UpdatesView);
            }
        },
        None => {
            print_res!(single pkgsite, index, views::index::IndexView);
        }
    };

    #[cfg(feature = "argh")]
    match args.subcommands {
        Some(cmd) => match cmd {
            Subcommands::Depends(Depends { packages }) => {
                print_res!(annotated pkgsite, depends, views::depends::DependsView, packages);
            }
            Subcommands::Rdepends(Rdepends { packages }) => {
                print_res!(annotated pkgsite, rdepends, views::rdepends::RDependsView, packages);
            }
            Subcommands::Show(Show { packages }) => {
                print_res!(unannotated pkgsite, info, views::info::InfoView, packages);
            }
            Subcommands::Search(Search {
                pattern,
                search_only,
            }) => {
                print_res!(single pkgsite, search, views::search::SearchView, &pattern, search_only);
            }
            Subcommands::Updates(_) => {
                print_res!(single pkgsite, updates, views::updates::UpdatesView);
            }
        },
        None => {
            print_res!(single pkgsite, index, views::index::IndexView);
        }
    };

    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

#[cfg(feature = "compio")]
#[compio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
