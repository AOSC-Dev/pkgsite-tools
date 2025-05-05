use console::style;
use pkgsite_lib::updates::Updates;
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use super::fmt_pkg_version;
use pkgsite_tools::PADDING;

pub struct UpdatesView<'a> {
    inner: &'a Updates,
}

impl<'a> From<&'a Updates> for UpdatesView<'a> {
    fn from(inner: &'a Updates) -> Self {
        Self { inner }
    }
}

impl Display for UpdatesView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut updates_table = Builder::default();
        for package in &self.inner.packages {
            updates_table.push_record([
                &package.name,
                &fmt_pkg_version(&package.dpkg_version, package.status, package.ver_compare)
                    .to_string(),
                &package.full_version,
            ]);
        }

        let updates_table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        write!(
            f,
            "Latest 100 Source Updates:
{}

NOTE:
{}
{}
{}
{}
{}",
            updates_table.build().with(updates_table_settings),
            style("Up to Date").green(),
            style("In Testing Branch").blue(),
            style("Older than Source / Missing").yellow(),
            style("Source File Contains Error").red(),
            style("Source Removed").blink(),
        )
    }
}
