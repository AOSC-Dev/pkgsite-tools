use anyhow::Result;
use console::style;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use pkgsite_tools::{PACKAGES_SITE_URL, PADDING};

#[derive(Debug, Serialize, Deserialize)]
struct PackageError {
    message: String,
    path: String,
    tree: String,
    branch: String,
}

impl Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}:{}) {}: {}",
            &self.tree, &self.branch, &self.path, &self.message
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DpkgMeta {
    hasmeta: bool,
}

impl Display for DpkgMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.hasmeta { '✓' } else { 'x' })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MatrixRow {
    repo: String,
    meta: Vec<DpkgMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    testing: bool,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    name: String,
    version: String,
    description: String,
    category: String,
    section: String,
    errors: Vec<PackageError>,
    srctype: String,
    srcurl_base: String,
    srcurl: String,
    full_version: String,
    versions: Vec<Version>,
    version_matrix: Vec<MatrixRow>,
}

impl Info {
    pub async fn fetch(packages: &[String]) -> Result<Vec<Self>> {
        let mut res = Vec::new();
        for package in packages.iter() {
            res.push(
                reqwest::get(format!(
                    "{}/packages/{}?type=json",
                    PACKAGES_SITE_URL, package
                ))
                .await?
                .json::<Self>()
                .await?,
            );
        }
        Ok(res)
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut version_matrix = Builder::default();
        version_matrix.push_record(
            vec!["Version".to_owned()].into_iter().chain(
                self.versions
                    .iter()
                    .map(|ver| {
                        let italic_version = style(&ver.version).italic().to_string();
                        if ver.testing {
                            italic_version
                        } else {
                            ver.version.clone()
                        }
                    })
                    .collect::<Vec<String>>(),
            ),
        );
        for row in &self.version_matrix {
            version_matrix.push_record(
                vec![row.repo.clone()].into_iter().chain(
                    row.meta
                        .iter()
                        .map(|meta| meta.to_string())
                        .collect::<Vec<String>>(),
                ),
            );
        }

        let table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        write!(
            f,
            "Package: {}
Version: {} ({})
Description: {}
Section: {}-{}
Upstream: {}
Source: ({}) {}{}

Available versions:
{}
{}",
            &self.name,
            &self.full_version,
            &self.version,
            &self.description,
            &self.category,
            &self.section,
            &self.srcurl_base,
            &self.srctype,
            &self.srcurl,
            if self.errors.is_empty() {
                String::new()
            } else {
                format!(
                    "\nErrors:\n{}",
                    &self
                        .errors
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            },
            version_matrix.build().with(table_settings),
            if self.versions.iter().any(|version| version.testing) {
                format!(
                    "\nNOTE: {} versions are italicized.",
                    style("Testing").italic(),
                )
            } else {
                String::new()
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        println!(
            "{:?}",
            Info::fetch(["wayland".to_owned()].as_slice())
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_display() {
        println!(
            "{}",
            Info::fetch(["wayland".to_owned()].as_slice())
                .await
                .unwrap()
                .first()
                .unwrap()
        );
    }
}
