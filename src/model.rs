use clap::ValueEnum;
use semver::{Prerelease, Version as SemVer};
use std::fmt::{Debug, Display, Formatter};
use time::{Date, OffsetDateTime};

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum Change {
    Major,
    Minor,
    Patch,
    Prerelease,
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

    pub fn bump(&mut self, new_version: &SemVer) -> bool {
        if let Some(latest_version) = self.version() {
            if let Some(unreleased) = self.unreleased() {
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

pub trait Bump {
    fn bump(&self, change: Change) -> Self;
}

impl Bump for SemVer {
    fn bump(&self, change: Change) -> Self {
        match change {
            Change::Major => {
                if self.pre.is_empty() || self.minor > 0 || self.patch > 0 {
                    SemVer::new(self.major + 1, 0, 0)
                } else {
                    SemVer::new(self.major, 0, 0)
                }
            }
            Change::Minor => {
                if self.pre.is_empty() || self.patch > 0 {
                    SemVer::new(self.major, self.minor + 1, 0)
                } else {
                    SemVer::new(self.major, self.minor, 0)
                }
            }
            Change::Patch => {
                if self.pre.is_empty() {
                    SemVer::new(self.major, self.minor, self.patch + 1)
                } else {
                    SemVer::new(self.major, self.minor, self.patch)
                }
            }
            Change::Prerelease => {
                if self.pre.is_empty() {
                    let mut next = SemVer::new(self.major, self.minor, self.patch + 1);
                    next.pre = Prerelease::new("next.0").unwrap();
                    next
                } else {
                    let mut next = SemVer::new(self.major, self.minor, self.patch);
                    let pre = self.pre.as_str();
                    let mut split = pre.rsplit('.');
                    let pre_index = split.next().unwrap().parse::<u64>().unwrap() + 1;
                    let pre_tag = split.next().unwrap();

                    next.pre = Prerelease::new(&format!("{pre_tag}.{pre_index}")).unwrap();
                    next
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_major() {
        let v_act = SemVer::parse("1.2.3").unwrap();
        let v_exp = SemVer::parse("2.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Major), v_exp);

        let v_act = SemVer::parse("0.0.0").unwrap();
        let v_exp = SemVer::parse("1.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Major), v_exp);

        let v_act = SemVer::parse("1.0.0-next.0").unwrap();
        let v_exp = SemVer::parse("1.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Major), v_exp);

        let v_act = SemVer::parse("1.0.1-next.0").unwrap();
        let v_exp = SemVer::parse("2.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Major), v_exp);

        let v_act = SemVer::parse("1.1.0-next.0").unwrap();
        let v_exp = SemVer::parse("2.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Major), v_exp);
    }

    #[test]
    fn bump_minor() {
        let v_act = SemVer::parse("1.2.3").unwrap();
        let v_exp = SemVer::parse("1.3.0").unwrap();
        assert_eq!(v_act.bump(Change::Minor), v_exp);

        let v_act = SemVer::parse("0.0.0").unwrap();
        let v_exp = SemVer::parse("0.1.0").unwrap();
        assert_eq!(v_act.bump(Change::Minor), v_exp);

        let v_act = SemVer::parse("1.0.1-next.0").unwrap();
        let v_exp = SemVer::parse("1.1.0").unwrap();
        assert_eq!(v_act.bump(Change::Minor), v_exp);

        let v_act = SemVer::parse("1.0.0-next.0").unwrap();
        let v_exp = SemVer::parse("1.0.0").unwrap();
        assert_eq!(v_act.bump(Change::Minor), v_exp);

        let v_act = SemVer::parse("1.1.0-next.0").unwrap();
        let v_exp = SemVer::parse("1.1.0").unwrap();
        assert_eq!(v_act.bump(Change::Minor), v_exp);
    }

    #[test]
    fn bump_patch() {
        let v_act = SemVer::parse("1.2.3").unwrap();
        let v_exp = SemVer::parse("1.2.4").unwrap();
        assert_eq!(v_act.bump(Change::Patch), v_exp);

        let v_act = SemVer::parse("0.0.0").unwrap();
        let v_exp = SemVer::parse("0.0.1").unwrap();
        assert_eq!(v_act.bump(Change::Patch), v_exp);

        let v_act = SemVer::parse("1.0.1-next.0").unwrap();
        let v_exp = SemVer::parse("1.0.1").unwrap();
        assert_eq!(v_act.bump(Change::Patch), v_exp);
    }

    #[test]
    fn bump_prerelease() {
        let v_act = SemVer::parse("1.2.3").unwrap();
        let v_exp = SemVer::parse("1.2.4-next.0").unwrap();
        assert_eq!(v_act.bump(Change::Prerelease), v_exp);

        let v_act = SemVer::parse("0.0.0").unwrap();
        let v_exp = SemVer::parse("0.0.1-next.0").unwrap();
        assert_eq!(v_act.bump(Change::Prerelease), v_exp);

        let v_act = SemVer::parse("1.0.1-next.0").unwrap();
        let v_exp = SemVer::parse("1.0.1-next.1").unwrap();
        assert_eq!(v_act.bump(Change::Prerelease), v_exp);
    }
}
