#[cfg(feature = "clap")]
use std::io::Result;

#[cfg(feature = "clap")]
include!("src/cli.rs");

#[cfg(feature = "clap")]
fn generate_completions() -> Result<()> {
    use clap::CommandFactory;
    use clap_complete::{Shell, generate_to};

    const ROOT: &str = "completions";
    const APP: &str = "pkgsite";
    const GENERATED_COMPLETIONS: &[Shell] = &[
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::Elvish,
        Shell::PowerShell,
    ];

    std::fs::create_dir_all(ROOT)?;
    let mut app = Cli::command();
    for shell in GENERATED_COMPLETIONS {
        generate_to(*shell, &mut app, APP, ROOT)?;
    }

    Ok(())
}

#[cfg(feature = "clap")]
fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=PKGSITE_GEN_COMPLETIONS");
    if std::env::var("PKGSITE_GEN_COMPLETIONS").is_ok() {
        generate_completions()?;
    }

    Ok(())
}

#[cfg(feature = "argh")]
fn main() {}
