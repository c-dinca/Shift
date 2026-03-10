use std::fs;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Package{
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct DetectStack{
    pub node_version: Option<String>,
    pub packages: Vec<Package>,
}

#[derive(Deserialize)]
pub struct Engines{
    pub node: Option<String>,
}
#[derive(Deserialize)]
pub struct PackageJson{
    pub dependencies: Option<HashMap<String, String>>,
    pub engines: Option<Engines>,
}

pub fn detect_stack() -> DetectStack {
    DetectStack{
        node_version: None,
        packages: vec![],
    }
}

pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

pub fn detect(path: String) -> Result<DetectStack, String> {
    let content = read_file(path)?;
    let pkg: PackageJson = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let node_version = pkg.engines
        .and_then(|e| e.node);

    let packages = pkg.dependencies
        .unwrap_or_default()
        .into_iter()
        .map(|(name, version)| Package{name, version})
        .collect();

    Ok(DetectStack{
        node_version,
        packages,
    })
}