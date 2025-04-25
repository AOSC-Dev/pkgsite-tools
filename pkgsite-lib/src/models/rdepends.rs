use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct RevDependency {
    package: String,
    version: String,
    architecture: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RevDependencyGroup {
    description: String,
    deps: Vec<RevDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RDepends {
    revdeps: Vec<RevDependencyGroup>,
    sobreaks: Vec<Vec<String>>,
    sobreaks_circular: Vec<String>,
    sorevdeps: HashMap<String, Vec<String>>,
}
