use anyhow::{Result, bail};
use console::style;
use regex::{Captures, Regex};
use reqwest::{Client, StatusCode, redirect::Policy};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use pkgsite_tools::{PACKAGES_SITE_URL, PADDING};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Package {
    name_highlight: String,
    full_version: String,
    desc_highlight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    packages: Vec<Package>,
}

impl Search {
    pub async fn fetch(pattern: &str, noredir: bool) -> Result<Box<dyn ToString>> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        let response = client
            .get(format!(
                "{}/search?q={}&type=json{}",
                PACKAGES_SITE_URL,
                pattern,
                if noredir { "&noredir=true" } else { "" }
            ))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(Box::new(response.json::<Self>().await?)),
            StatusCode::SEE_OTHER => {
                let package = response
                    .headers()
                    .get("location")
                    .unwrap()
                    .to_str()?
                    .strip_prefix("/packages/")
                    .unwrap()
                    .to_string();
                Ok(Box::new(format!(
                    "Found an exact match:\n(append --search-only to search the keyword instead)\n\n{}",
                    super::info::Info::fetch(&[package])
                        .await?
                        .iter()
                        .map(|res| res.to_string())
                        .collect::<Vec<String>>()
                        .join("\n\n"),
                )))
            }
            _ => bail!("Error searching for packages"),
        }
    }
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let highlight_regex = Regex::new(r"<b>(?<highlight>.+?)<\/b>").unwrap();
        let highlight_rep = |caps: &Captures| -> String {
            style(&caps["highlight"]).bold().underlined().to_string()
        };
        let packages = &self
            .packages
            .iter()
            .map(|pkg| Package {
                name_highlight: highlight_regex
                    .replace_all(&pkg.name_highlight, highlight_rep)
                    .to_string(),
                desc_highlight: html_escape::decode_html_entities(
                    &highlight_regex
                        .replace_all(&pkg.desc_highlight, highlight_rep)
                        .to_string(),
                )
                .to_string(),
                ..(*pkg).clone()
            })
            .collect::<Vec<Package>>();

        let mut packages_table = Builder::default();
        for package in packages {
            packages_table.push_record([
                &package.name_highlight,
                &package.full_version,
                &package.desc_highlight,
            ]);
        }

        let table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        write!(f, "{}", packages_table.build().with(table_settings))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        println!(
            "{:?}",
            Search::fetch("-base", false).await.unwrap().to_string()
        );
    }

    #[tokio::test]
    async fn test_display() {
        println!(
            "{}",
            Search::fetch("-base", false).await.unwrap().to_string()
        );
    }
}
