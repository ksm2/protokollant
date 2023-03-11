use clap::ValueEnum;
use semver::{Prerelease, Version as SemVer};
use std::fmt::{Debug, Display, Formatter};
use time::{Date, OffsetDateTime};

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum Change {
    Major,
    Minor,
    Patch,
    NextIteration,
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

    pub fn bump(&mut self, change: Change) -> bool {
        if let Some(latest_version) = self.version() {
            if let Some(unreleased) = self.unreleased() {
                let new_version = Self::bump_version(&latest_version, change);
                unreleased.version = Version::Released(new_version.clone());
                unreleased.date = Some(OffsetDateTime::now_local().unwrap().date());

                if let Some(mut old_ref) = self.refs.first_mut() {
                    let old_version_string = latest_version.to_string();
                    let version_string = new_version.to_string();
                    let href = old_ref.href.clone();
                    old_ref.href = href.replace(&old_version_string, &version_string);

                    let new_ref = Ref::new(
                        version_string,
                        href.replace("HEAD", &format!("v{new_version}")),
                    );
                    self.refs.insert(1, new_ref);
                }
                return true;
            }
        }
        false
    }

    fn bump_version(version: &SemVer, change: Change) -> SemVer {
        match change {
            Change::Major => SemVer::new(version.major + 1, 0, 0),
            Change::Minor => SemVer::new(version.major, version.minor + 1, 0),
            Change::Patch => SemVer::new(version.major, version.minor, version.patch + 1),
            Change::NextIteration => {
                let mut next = SemVer::new(version.major, version.minor, version.patch + 1);
                if version.pre.is_empty() {
                    next.pre = Prerelease::new("next.0").unwrap();
                } else {
                    let pre = version.pre.as_str();
                    let mut split = pre.rsplit('.');
                    let pre_index = split.next().unwrap().parse::<u64>().unwrap() + 1;
                    let pre_tag = split.next().unwrap();

                    next.pre = Prerelease::new(&format!("{pre_tag}.{pre_index}")).unwrap();
                }
                next
            }
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

    pub fn has_unreleased(&self) -> bool {
        self.releases
            .iter()
            .any(|rel| rel.version == Version::Unreleased)
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
