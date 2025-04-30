use pkgsite_lib::depends::{Dependency, Depends};
use std::fmt::Display;

struct DependencyView<'a> {
    inner: &'a Dependency,
}

impl<'a> From<&'a Dependency> for DependencyView<'a> {
    fn from(inner: &'a Dependency) -> Self {
        Self { inner }
    }
}

impl Display for DependencyView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {}",
            &self.inner.relationship,
            if self.inner.arch.is_empty() {
                String::new()
            } else {
                format!(" {}", &self.inner.arch)
            },
            &self
                .inner
                .packages
                .iter()
                .map(|(pkg, ver)| { format!("{}{}", pkg, ver) })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct DependsView<'a> {
    inner: &'a Depends,
}

impl<'a> From<&'a Depends> for DependsView<'a> {
    fn from(inner: &'a Depends) -> Self {
        Self { inner }
    }
}

impl Display for DependsView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}
Library Dependencies: {}",
            &self
                .inner
                .dependencies
                .iter()
                .map(|dep| DependencyView::from(dep).to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            &self.inner.library_dependencies.join(", ")
        )
    }
}
