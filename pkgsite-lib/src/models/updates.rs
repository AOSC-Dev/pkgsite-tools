use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub dpkg_version: String,
    pub description: String,
    pub full_version: String,
    pub ver_compare: i32,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Updates {
    pub packages: Vec<Package>,
}
