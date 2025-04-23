use anyhow::Result;
use console::{Alignment, measure_text_width, pad_str, style};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::{UtcDateTime, format_description};

use pkgsite_tools::{PACKAGES_SITE_URL, PADDING};

const REPOSITORY_HEADERS: [&str; 5] = ["Repository", "Count", "Ghost", "Lagging", "Missing"];

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
        let max_pkgname_width = &self
            .updates
            .iter()
            .map(|pkg| measure_text_width(&pkg.name))
            .max()
            .unwrap_or(10);
        let max_version_width = &self
            .updates
            .iter()
            .map(|pkg| measure_text_width(&pkg.full_version))
            .max()
            .unwrap_or(10);

        let (_, repositories) = &self.repo_categories.first().unwrap();

        let max_repo_properties_widths = repositories
            .iter()
            .map(|repo| {
                [
                    measure_text_width(&repo.realname),
                    measure_text_width(&repo.pkgcount.to_string()),
                    measure_text_width(&repo.ghost.to_string()),
                    measure_text_width(&repo.lagging.to_string()),
                    measure_text_width(&repo.missing.to_string()),
                ]
            })
            .fold(
                REPOSITORY_HEADERS
                    .iter()
                    .map(|s| measure_text_width(*s))
                    .collect::<Vec<usize>>(),
                |acc, x| {
                    vec![
                        acc[0].max(x[0]),
                        acc[1].max(x[1]),
                        acc[2].max(x[2]),
                        acc[3].max(x[3]),
                        acc[4].max(x[4]),
                    ]
                },
            );

        write!(
            f,
            "AOSC OS has a total of {} packages.

Latest Source Updates:
{}

Repositories:
{}{}{}{}{}{}
{}",
            &self.total,
            &self
                .updates
                .iter()
                .map(|pkg| {
                    let italic_version = style(&pkg.full_version).italic().to_string();
                    format!(
                        "{}{}{}",
                        pad_str(
                            &pkg.name,
                            max_pkgname_width + PADDING,
                            Alignment::Left,
                            None
                        ),
                        pad_str(
                            if pkg.status == 1 {
                                &italic_version
                            } else {
                                &pkg.full_version
                            },
                            max_version_width + PADDING,
                            Alignment::Left,
                            None
                        ),
                        &pkg.description,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            pad_str(
                "Repository",
                max_repo_properties_widths[0] + PADDING,
                Alignment::Left,
                None
            ),
            pad_str(
                "Count",
                max_repo_properties_widths[1] + PADDING,
                Alignment::Left,
                None
            ),
            pad_str(
                "Ghost",
                max_repo_properties_widths[2] + PADDING,
                Alignment::Left,
                None
            ),
            pad_str(
                "Lagging",
                max_repo_properties_widths[3] + PADDING,
                Alignment::Left,
                None
            ),
            pad_str(
                "Missing",
                max_repo_properties_widths[4] + PADDING,
                Alignment::Left,
                None
            ),
            "Last Update (UTC)",
            &self
                .repo_categories
                .first()
                .unwrap()
                .1
                .iter()
                .map(|repo| {
                    let datetime = UtcDateTime::from_unix_timestamp(repo.date.into())
                        .unwrap()
                        .format(
                            &format_description::parse("[year]-[month]-[day] [hour]:[minute]")
                                .unwrap(),
                        )
                        .unwrap_or(repo.date.to_string());
                    format!(
                        "{}{}{}{}{}{}",
                        pad_str(
                            &repo.realname,
                            max_repo_properties_widths[0] + PADDING,
                            Alignment::Left,
                            None
                        ),
                        pad_str(
                            &repo.pkgcount.to_string(),
                            max_repo_properties_widths[1] + PADDING,
                            Alignment::Left,
                            None
                        ),
                        pad_str(
                            &repo.ghost.to_string(),
                            max_repo_properties_widths[2] + PADDING,
                            Alignment::Left,
                            None
                        ),
                        pad_str(
                            &repo.lagging.to_string(),
                            max_repo_properties_widths[3] + PADDING,
                            Alignment::Left,
                            None
                        ),
                        pad_str(
                            &repo.missing.to_string(),
                            max_repo_properties_widths[4] + PADDING,
                            Alignment::Left,
                            None
                        ),
                        datetime
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
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
