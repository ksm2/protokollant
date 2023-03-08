use crate::model::{Changelog, Ref, Release, Version};
use pest::iterators::Pair;
use pest::Parser;
use semver::Version as SemVer;
use std::fs::read_to_string;
use std::io::Result;

#[derive(Parser)]
#[grammar = "changelog.pest"]
struct ChangelogParser;

pub fn parse_file(filename: &str) -> Result<Changelog> {
    let content = read_to_string(filename)?;
    Ok(parse_str(&content))
}

pub fn parse_str(content: &str) -> Changelog {
    let mut changelog = Changelog::new();
    let parsed = ChangelogParser::parse(Rule::Changelog, content)
        .expect("failed to parse file")
        .next()
        .unwrap();
    for line in parsed.into_inner() {
        match line.as_rule() {
            Rule::Intro => {
                changelog.intro = line.as_str().into();
            }
            Rule::Unreleased => {
                let version = parse_release(line);
                changelog.releases.push(version);
            }
            Rule::Release => {
                let version = parse_release(line);
                changelog.releases.push(version);
            }
            Rule::Reference => {
                let mut inner_rules = line.into_inner();

                let anchor = inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str();
                let href = inner_rules.next().unwrap().as_str();

                let r = Ref::new(anchor.into(), href.into());
                changelog.refs.push(r);
            }
            _ => {
                // do nothing
            }
        }
    }
    changelog
}

fn parse_release(release_rules: Pair<Rule>) -> Release {
    let mut v = Release::default();
    for line in release_rules.into_inner() {
        match line.as_rule() {
            Rule::ReleaseIntro => {
                v.intro = line.as_str().into();
            }
            Rule::UnreleasedHeading => {
                v.version = Version::Unreleased;
            }
            Rule::ReleaseHeading => {
                let mut inner_rules = line.into_inner();
                let version = inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str();
                let date = inner_rules.next().unwrap().as_str();

                v.version = Version::Released(SemVer::parse(version).unwrap());
                v.date = Some(date.into());
            }
            Rule::Section => {
                let mut inner_rules = line.into_inner();
                let sec = inner_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_lowercase();
                let bullets = inner_rules.next().unwrap();

                let bullets = bullets
                    .into_inner()
                    .map(|bullet| {
                        bullet
                            .into_inner()
                            .map(|each| each.into_inner().next().unwrap().as_str())
                            .fold(String::new(), |a, b| a + b + "\n")
                            .trim()
                            .to_string()
                    })
                    .collect::<Vec<_>>();

                if sec == "added" {
                    v.added = bullets;
                } else if sec == "removed" {
                    v.removed = bullets;
                } else if sec == "fixed" {
                    v.fixed = bullets;
                } else if sec == "changed" {
                    v.changed = bullets;
                }
            }
            _ => {
                // Do nothing
            }
        }
    }

    v
}
