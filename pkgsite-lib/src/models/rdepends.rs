use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RevDependency {
    pub package: String,
    pub version: String,
    pub architecture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevDependencyGroup {
    pub description: String,
    pub deps: Vec<RevDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RDepends {
    pub revdeps: Vec<RevDependencyGroup>,
    pub sobreaks: Vec<Vec<String>>,
    pub sobreaks_circular: Vec<String>,
    pub sorevdeps: HashMap<String, Vec<String>>,
}
