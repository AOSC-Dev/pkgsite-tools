use console::style;
use pkgsite_lib::info::{DpkgMeta, Info};
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use pkgsite_tools::PADDING;

struct DpkgMetaView<'a> {
    inner: &'a DpkgMeta,
}

impl<'a> From<&'a DpkgMeta> for DpkgMetaView<'a> {
    fn from(inner: &'a DpkgMeta) -> Self {
        Self { inner }
    }
}

impl Display for DpkgMetaView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.inner.hasmeta { '✓' } else { 'x' })
    }
}

pub struct InfoView<'a> {
    inner: &'a Info,
}

impl<'a> From<&'a Info> for InfoView<'a> {
    fn from(inner: &'a Info) -> Self {
        Self { inner }
    }
}

impl Display for InfoView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut version_matrix = Builder::default();
        version_matrix.push_record(
            vec!["Version".to_owned()].into_iter().chain(
                self.inner
                    .versions
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
        for row in &self.inner.version_matrix {
            version_matrix.push_record(
                vec![row.repo.clone()].into_iter().chain(
                    row.meta
                        .iter()
                        .map(|meta| DpkgMetaView::from(meta).to_string())
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
Source: ({}) {}

Available versions:
{}
{}",
            &self.inner.name,
            &self.inner.full_version,
            &self.inner.version,
            &self.inner.description,
            &self.inner.category,
            &self.inner.section,
            &self.inner.srcurl_base,
            &self.inner.srctype,
            &self.inner.srcurl,
            version_matrix.build().with(table_settings),
            if self.inner.versions.iter().any(|version| version.testing) {
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
