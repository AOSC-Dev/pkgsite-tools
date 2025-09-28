pub mod depends;
pub mod files;
pub mod index;
pub mod info;
pub mod rdepends;
pub mod search;
pub mod updates;

use console::{StyledObject, style};

fn fmt_pkg_version(version: &str, status: i32, ver_compare: i32) -> StyledObject<&str> {
    let styled_ver = style(version);
    match status {
        1 => styled_ver.red(),
        2 => styled_ver.blue(),
        _ => match ver_compare {
            -2 => styled_ver.blink(),
            -1 => styled_ver.yellow(),
            0 => styled_ver.green(),
            1 => styled_ver.blue(),
            _ => styled_ver,
        },
    }
}
