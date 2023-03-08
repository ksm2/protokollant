use clap::ValueEnum;
use semver::Version as SemVer;
use std::fmt::{Debug, Display, Formatter};
use time::{Date, OffsetDateTime};

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum Change {
    Major,
    Minor,
    Patch,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Changelog {
    pub intro: String,
    pub releases: Vec<Release>,
    pub refs: Vec<Ref>,
}

impl Changelog {
    pub fn new() -> Self {
        Self {
            intro: String::new(),
            releases: Vec::new(),
            refs: Vec::new(),
        }
    }

    pub fn bump(&mut self, change: Change) {
        if let Some(latest_version) = self.version() {
            if let Some(unreleased) = self.unreleased() {
                let new_version = Self::bump_version(latest_version, change);
                unreleased.version = Version::Released(new_version);
                unreleased.date = Some(OffsetDateTime::now_local().unwrap().date())
            }
        }
    }

    fn bump_version(version: SemVer, change: Change) -> SemVer {
        match change {
            Change::Major => SemVer::new(version.major + 1, 0, 0),
            Change::Minor => SemVer::new(version.major, version.minor + 1, 0),
            Change::Patch => SemVer::new(version.major, version.minor, version.patch + 1),
        }
    }

    pub fn version(&self) -> Option<SemVer> {
        self.releases
            .iter()
            .filter_map(|r| match &r.version {
                Version::Unreleased => None,
                Version::Released(s) => Some(s),
            })
            .max()
            .cloned()
    }

    pub fn unreleased(&mut self) -> Option<&mut Release> {
        self.releases
            .iter_mut()
            .find(|r| matches!(r.version, Version::Unreleased))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Release {
    pub version: Version,
    pub date: Option<Date>,
    pub intro: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub fixed: Vec<String>,
    pub changed: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Version {
    #[default]
    Unreleased,
    Released(SemVer),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Unreleased => f.write_str("Unreleased"),
            Version::Released(v) => Display::fmt(v, f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Ref {
    pub anchor: String,
    pub href: String,
}

impl Ref {
    pub fn new(anchor: String, href: String) -> Self {
        Self { anchor, href }
    }
}
