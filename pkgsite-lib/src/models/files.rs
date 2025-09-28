use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub filename: Option<String>,
    pub size: i64,
    pub ftype: i16,
    pub perm: i32,
    pub uid: i64,
    pub gid: i64,
    pub uname: String,
    pub gname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub repo: String,
    pub maintainer: String,
    pub installed_size: i64,
    pub filename: String,
    pub size: i64,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
    pub sodepends: Vec<String>,
    pub soprovides: Vec<String>,
    pub pkg_debtime: i32,
    pub pkg: Package,
}
