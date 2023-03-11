use crate::diff::FileDiff;
use semver::Version;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, write};
use std::io::Result;
use std::path::Path;
use toml_edit::{value, Document};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ManifestType {
    Cargo,
}

impl ManifestType {
    pub fn change_version(&self, version: &Version, do_write: bool) -> Result<Vec<FileDiff>> {
        match self {
            ManifestType::Cargo => self.change_cargo_version(version, do_write),
        }
    }

    fn change_cargo_version(&self, version: &Version, do_write: bool) -> Result<Vec<FileDiff>> {
        let (toml, name) = self.change_cargo_toml_version(version, do_write)?;
        let lock = self.change_cargo_lock_version(&name, version, do_write)?;
        Ok(vec![lock, toml])
    }

    fn change_cargo_toml_version(
        &self,
        version: &Version,
        do_write: bool,
    ) -> Result<(FileDiff, String)> {
        let old_toml = read_to_string("Cargo.toml")?;
        let mut manifest = old_toml.parse::<Document>().unwrap();
        manifest["package"]["version"] = value(version.to_string());
        let name = manifest["package"]["name"].as_str().unwrap().to_string();
        let new_toml = manifest.to_string();
        if do_write {
            write("Cargo.toml", &new_toml)?;
        }

        let diff = FileDiff::new("Cargo.toml", old_toml, new_toml);
        Ok((diff, name))
    }

    fn change_cargo_lock_version(
        &self,
        name: &str,
        version: &Version,
        do_write: bool,
    ) -> Result<FileDiff> {
        let old_toml = read_to_string("Cargo.lock")?;
        let mut manifest = old_toml.parse::<Document>().unwrap();
        let packages = manifest["package"].as_array_of_tables_mut().unwrap();
        for package in packages.iter_mut() {
            if package["name"].as_str() == Some(name) {
                package["version"] = value(version.to_string());
                break;
            }
        }
        let new_toml = manifest.to_string();
        if do_write {
            write("Cargo.lock", &new_toml)?;
        }

        let diff = FileDiff::new("Cargo.lock", old_toml, new_toml);
        Ok(diff)
    }
}

impl Display for ManifestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestType::Cargo => f.write_str("Rust (Cargo.lock, Cargo.toml)"),
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
