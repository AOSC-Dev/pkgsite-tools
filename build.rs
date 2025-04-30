use clap::CommandFactory;
use clap_complete::{Shell, generate_to};
use std::{env, fs, io::Result};

include!("src/cli.rs");

const ROOT: &str = "completions";
const APP: &str = "pkgsite";
const GENERATED_COMPLETIONS: &[Shell] = &[
    Shell::Bash,
    Shell::Zsh,
    Shell::Fish,
    Shell::Elvish,
    Shell::PowerShell,
];

fn generate_completions() -> Result<()> {
    fs::create_dir_all(ROOT)?;
    let mut app = Cli::command();
    for shell in GENERATED_COMPLETIONS {
        generate_to(*shell, &mut app, APP, ROOT)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=PKGSITE_GEN_COMPLETIONS");
    if env::var("PKGSITE_GEN_COMPLETIONS").is_ok() {
        generate_completions()?;
    }

    Ok(())
}
