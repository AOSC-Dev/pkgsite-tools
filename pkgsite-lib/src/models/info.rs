use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageError {
    pub message: String,
    pub path: String,
    pub tree: String,
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DpkgMeta {
    pub hasmeta: bool,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixRow {
    pub repo: String,
    pub meta: Vec<DpkgMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub testing: bool,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub section: String,
    pub errors: Vec<PackageError>,
    pub srctype: String,
    pub srcurl_base: String,
    pub srcurl: String,
    pub full_version: String,
    pub versions: Vec<Version>,
    pub version_matrix: Vec<MatrixRow>,
}
