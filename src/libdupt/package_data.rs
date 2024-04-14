use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PackageData {
    pub package_name: String,
    pub name: String,
    pub owner: String,
    pub summary: String,
    pub description: String,
    pub version: String,
    pub make_dependencies: Vec<String>,
    pub dependencies: Vec<String>,
    pub exec_dir: String
}