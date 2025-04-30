use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    pub name_highlight: String,
    pub full_version: String,
    pub desc_highlight: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    pub packages: Vec<Package>,
}
