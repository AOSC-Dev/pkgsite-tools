use pkgsite_lib::files::Files;
use std::fmt::Display;
use tabled::{
    builder::Builder,
    settings::{Alignment, Modify, Padding, Settings, Style, object::SegmentAll},
};

use pkgsite_tools::PADDING;

fn ls_perm(perm: &i32, ftype: &i16) -> String {
    // see https://docs.rs/tar/latest/src/tar/entry_type.rs.html#70-87
    let ftype = match ftype {
        1 => 'l',
        3 => 'c',
        4 => 'b',
        5 => 'd',
        6 => 'p',
        _ => '-',
    };

    let perm: String = format!("{perm:b}")
        .chars()
        .zip("rwxrwxrwx".chars())
        .map(|(a, b)| if a == '1' { b } else { '-' })
        .collect();

    format!("{ftype}{perm}")
}

pub struct FilesView<'a> {
    inner: &'a Files,
}

impl<'a> From<&'a Files> for FilesView<'a> {
    fn from(inner: &'a Files) -> Self {
        Self { inner }
    }
}

impl Display for FilesView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut file_matrix = Builder::default();
        for row in &self.inner.files {
            file_matrix.push_record(vec![
                ls_perm(&row.perm, &row.ftype),
                format!("{}({})", row.uname, row.uid),
                format!("{}({})", row.gname, row.gid),
                row.size.to_string(),
                row.filename.clone().unwrap_or_default(),
            ]);
        }

        let table_settings = Settings::default().with(Style::blank()).with(
            Modify::new(SegmentAll)
                .with(Alignment::left())
                .with(Padding::new(0, PADDING, 0, 0)),
        );

        write!(
            f,
            "Files in \"{}\" ({})
Repository: {}/{}
Package Time: {}
Size: {}
SHA256: {}
Installed Size: {}
Maintainer: {}{}{}

Files:
{}",
            &self.inner.pkg.package,
            &self.inner.pkg.version,
            &self.inner.pkg.architecture,
            &self.inner.pkg.repo,
            &self.inner.pkg_debtime,
            &self.inner.pkg.size,
            &self.inner.pkg.sha256,
            &self.inner.pkg.installed_size,
            &self.inner.pkg.maintainer,
            if self.inner.sodepends.is_empty() {
                String::new()
            } else {
                format!("\nLibrary Depends: {}", &self.inner.sodepends.join(", "))
            },
            if self.inner.soprovides.is_empty() {
                String::new()
            } else {
                format!("\nLibrary Provides: {}", &self.inner.soprovides.join(", "))
            },
            file_matrix.build().with(table_settings),
        )
    }
}
