use crate::diff::FileDiff;
use semver::Version;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;
use toml::{to_string_pretty, Table, Value};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ManifestType {
    Cargo,
}

impl ManifestType {
    pub fn change_version(&self, version: &Version) -> Result<FileDiff> {
        match self {
            ManifestType::Cargo => self.change_cargo_version(version),
        }
    }

    fn change_cargo_version(&self, version: &Version) -> Result<FileDiff> {
        let old_toml = read_to_string("Cargo.toml")?;
        let mut manifest = old_toml.parse::<Table>().unwrap();
        let package = manifest["package"].as_table_mut().unwrap();
        package["version"] = Value::String(version.to_string());
        let new_toml = to_string_pretty(&manifest).unwrap();

        let diff = FileDiff::new("Cargo.toml", old_toml, new_toml);
        Ok(diff)
    }
}

impl Display for ManifestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestType::Cargo => f.write_str("Rust (Cargo.toml)"),
        }
    }
}

pub fn detect_manifests() -> Result<Vec<ManifestType>> {
    let mut manifests = Vec::new();

    if Path::new("Cargo.toml").exists() {
        manifests.push(ManifestType::Cargo);
    }

    Ok(manifests)
}
