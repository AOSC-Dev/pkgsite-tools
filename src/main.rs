use anyhow::Result;
use clap::Parser;
use pkgsite_lib::PackagesSiteClient;

mod cli;
mod views;

use cli::*;
use pkgsite_tools::{dedup_packages, print_res};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let pkgsite = PackagesSiteClient::default();

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

    Ok(())
}
