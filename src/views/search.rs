use console::style;
use pkgsite_lib::{SearchExactMatch, info::Info, search::Package};
use regex::{Captures, Regex};
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use super::info::InfoView;
use pkgsite_tools::PADDING;

pub struct SearchView<'a> {
    pub inner: &'a SearchExactMatch,
}

impl<'a> From<&'a SearchExactMatch> for SearchView<'a> {
    fn from(inner: &'a SearchExactMatch) -> Self {
        Self { inner }
    }
}

impl Display for SearchView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            SearchExactMatch::Search(search) => {
                let highlight_regex = Regex::new(r"<b>(?<highlight>.+?)<\/b>").unwrap();
                let highlight_rep = |caps: &Captures| -> String {
                    style(&caps["highlight"]).bold().underlined().to_string()
                };
                let packages = search
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
            SearchExactMatch::Info(info) => {
                write!(
                    f,
                    "Found an exact match:\n(append --search-only to search the keyword instead)\n\n{}",
                    InfoView::from(info as &Info)
                )
            }
        }
    }
}
