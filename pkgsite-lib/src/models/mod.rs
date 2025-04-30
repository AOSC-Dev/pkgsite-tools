pub mod depends;
pub mod index;
pub mod info;
pub mod rdepends;
pub mod search;

pub enum SearchExactMatch {
    Search(search::Search),
    Info(Box<info::Info>),
}
