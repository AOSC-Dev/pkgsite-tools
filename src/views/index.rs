use console::style;
use pkgsite_lib::index::Index;
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};
use time::{UtcDateTime, format_description};

use pkgsite_tools::PADDING;

const REPOSITORY_HEADERS: [&str; 6] = [
    "Repository",
    "Count",
    "Ghost",
    "Lagging",
    "Missing",
    "Last Update (UTC)",
];

pub struct IndexView<'a> {
    inner: &'a Index,
}

impl<'a> From<&'a Index> for IndexView<'a> {
    fn from(inner: &'a Index) -> Self {
        Self { inner }
    }
}

impl Display for IndexView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, repositories) = &self.inner.repo_categories.first().unwrap();

        let mut updates_table = Builder::default();
        for package in &self.inner.updates {
            let mut ver = match package.status {
                1 => style(&package.full_version).red(),
                2 => style(&package.full_version).blue(),
                _ => style(&package.full_version),
            };

            if ![1, 2].contains(&package.status) {
                ver = match package.ver_compare {
                    -2 => ver.blink(),
                    -1 => ver.yellow(),
                    0 => ver.green(),
                    1 => ver.blue(),
                    _ => ver,
                };
            }

            updates_table.push_record([&package.name, &ver.to_string(), &package.description]);
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

NOTE:
{}
{}
{}
{}
{}

Repositories:
{}",
            &self.inner.total,
            updates_table.build().with(updates_table_settings),
            style("Up to Date").green(),
            style("In Testing Branch").blue(),
            style("Older than Source / Missing").yellow(),
            style("Source File Contains Error").red(),
            style("Source Removed").blink(),
            repositories_table.build().with(repositories_table_settings),
        )
    }
}
