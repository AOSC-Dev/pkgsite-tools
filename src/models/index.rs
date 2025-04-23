use anyhow::Result;
use console::style;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};
use time::{UtcDateTime, format_description};

use pkgsite_tools::{PACKAGES_SITE_URL, PADDING};

const REPOSITORY_HEADERS: [&str; 6] = [
    "Repository",
    "Count",
    "Ghost",
    "Lagging",
    "Missing",
    "Last Update (UTC)",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repo {
    realname: String,
    date: i32,
    pkgcount: i32,
    ghost: i32,
    lagging: i32,
    missing: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    description: String,
    full_version: String,
    status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    total: i64,
    repo_categories: Vec<(String, Vec<Repo>)>,
    updates: Vec<Package>,
}

impl Index {
    pub async fn fetch() -> Result<Self> {
        Ok(reqwest::get(format!("{}/?type=json", PACKAGES_SITE_URL))
            .await?
            .json::<Self>()
            .await?)
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, repositories) = &self.repo_categories.first().unwrap();

        let mut updates_table = Builder::default();
        for package in &self.updates {
            let italic_version = style(&package.full_version).italic().to_string();
            updates_table.push_record([
                &package.name,
                if package.status == 1 {
                    &italic_version
                } else {
                    &package.full_version
                },
                &package.description,
            ]);
        }

        let mut repositories_table = Builder::default();
        repositories_table.push_record(REPOSITORY_HEADERS);
        for repository in repositories {
            repositories_table.push_record([
                &repository.realname,
                &repository.pkgcount.to_string(),
                &repository.ghost.to_string(),
                &repository.lagging.to_string(),
                &repository.missing.to_string(),
                &UtcDateTime::from_unix_timestamp(repository.date.into())
                    .unwrap_or(UtcDateTime::UNIX_EPOCH)
                    .format(
                        &format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap(),
                    )
                    .unwrap_or(repository.date.to_string()),
            ]);
        }

        let updates_table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        let repositories_table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        write!(
            f,
            "AOSC OS has a total of {} packages.

Latest Source Updates:
{}

Repositories:
{}",
            &self.total,
            updates_table.build().with(updates_table_settings),
            repositories_table.build().with(repositories_table_settings),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        println!("{:?}", Index::fetch().await.unwrap());
    }

    #[tokio::test]
    async fn test_display() {
        println!("{}", Index::fetch().await.unwrap());
    }
}
