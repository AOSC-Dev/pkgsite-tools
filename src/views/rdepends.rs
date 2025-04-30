use pkgsite_lib::rdepends::{RDepends, RevDependency, RevDependencyGroup};
use std::fmt::Display;

struct RevDependencyView<'a> {
    inner: &'a RevDependency,
}

impl<'a> From<&'a RevDependency> for RevDependencyView<'a> {
    fn from(inner: &'a RevDependency) -> Self {
        Self { inner }
    }
}

impl Display for RevDependencyView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            &self.inner.package,
            &self.inner.version,
            if self.inner.architecture.is_empty() {
                String::new()
            } else {
                format!(" [{}]", &self.inner.architecture)
            }
        )
    }
}

struct RevDependencyGroupView<'a> {
    inner: &'a RevDependencyGroup,
}

impl<'a> From<&'a RevDependencyGroup> for RevDependencyGroupView<'a> {
    fn from(inner: &'a RevDependencyGroup) -> Self {
        Self { inner }
    }
}

impl Display for RevDependencyGroupView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            &self.inner.description,
            &self
                .inner
                .deps
                .iter()
                .map(|pkg| RevDependencyView::from(pkg).to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct RDependsView<'a> {
    inner: &'a RDepends,
}

impl<'a> From<&'a RDepends> for RDependsView<'a> {
    fn from(inner: &'a RDepends) -> Self {
        Self { inner }
    }
}

impl Display for RDependsView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.inner.revdeps.is_empty() {
                String::new()
            } else {
                self.inner
                    .revdeps
                    .iter()
                    .map(|revdep| RevDependencyGroupView::from(revdep).to_string())
                    .collect::<Vec<String>>()
                    .join("\n\n")
            },
            if self.inner.sobreaks.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\nLibrary depended by:\n{}",
                    &self
                        .inner
                        .sobreaks
                        .iter()
                        .map(|sobreak| format!("- {}", sobreak.join(", ")))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            },
            if self.inner.sobreaks_circular.is_empty() {
                String::new()
            } else {
                format!(
                    "\n- (Circular dependencies) {}",
                    &self.inner.sobreaks_circular.join(", ")
                )
            },
            if self.inner.sorevdeps.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\nReverse dependencies of the libraries:\n{}",
                    &self
                        .inner
                        .sorevdeps
                        .iter()
                        .map(|(lib, revdep)| format!("- {}: {}", lib, revdep.join(", ")))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
        )
    }
}
