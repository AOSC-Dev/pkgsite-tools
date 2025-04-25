use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub relationship: String,
    pub arch: String,
    pub packages: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depends {
    pub dependencies: Vec<Dependency>,
    pub library_dependencies: Vec<String>,
}
