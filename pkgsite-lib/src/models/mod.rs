pub mod depends;
pub mod files;
pub mod index;
pub mod info;
pub mod rdepends;
pub mod search;
pub mod updates;

pub enum SearchOrInfo {
    Search(search::Search),
    Info(Box<info::Info>),
}
