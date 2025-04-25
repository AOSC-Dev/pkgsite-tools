use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repo {
    pub realname: String,
    pub date: i32,
    pub pkgcount: i32,
    pub ghost: i32,
    pub lagging: i32,
    pub missing: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub full_version: String,
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub total: i64,
    pub repo_categories: Vec<(String, Vec<Repo>)>,
    pub updates: Vec<Package>,
}
