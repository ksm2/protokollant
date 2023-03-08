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
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Release {
    pub version: String,
    pub date: Option<String>,
    pub intro: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub fixed: Vec<String>,
    pub changed: Vec<String>,
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
