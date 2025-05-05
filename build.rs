#[cfg(feature = "clap")]
use clap::CommandFactory;
#[cfg(feature = "clap")]
use clap_complete::{Shell, generate_to};
#[cfg(feature = "clap")]
use std::{env, fs, io::Result};

#[cfg(feature = "clap")]
include!("src/cli.rs");

#[cfg(feature = "clap")]
const ROOT: &str = "completions";
#[cfg(feature = "clap")]
const APP: &str = "pkgsite";
#[cfg(feature = "clap")]
const GENERATED_COMPLETIONS: &[Shell] = &[
    Shell::Bash,
    Shell::Zsh,
    Shell::Fish,
    Shell::Elvish,
    Shell::PowerShell,
];

#[cfg(feature = "clap")]
fn generate_completions() -> Result<()> {
    fs::create_dir_all(ROOT)?;
    let mut app = Cli::command();
    for shell in GENERATED_COMPLETIONS {
        generate_to(*shell, &mut app, APP, ROOT)?;
    }

    Ok(())
}

#[cfg(feature = "clap")]
fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=PKGSITE_GEN_COMPLETIONS");
    if env::var("PKGSITE_GEN_COMPLETIONS").is_ok() {
        generate_completions()?;
    }

    Ok(())
}

#[cfg(feature = "argh")]
fn main() {}
