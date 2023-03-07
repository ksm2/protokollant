use crate::model::{Changelog, Ref, Version};
use std::fs::write;
use std::io::Result;

pub fn generate_file(filename: &str, model: &Changelog) -> Result<()> {
    let str = generate_str(model);
    write(filename, str)?;
    Ok(())
}

pub fn generate_str(model: &Changelog) -> String {
    let mut str = String::new();

    str.push_str(&model.intro);

    for version in &model.versions {
        let Version {
            version,
            date,
            intro,
            added,
            removed,
            changed,
            fixed,
        } = version;
        if let Some(date) = date {
            str.push_str(&format!("## [{version}] - {date}\n\n"));
        } else {
            str.push_str(&format!("## [{version}]\n\n"));
        }
        str.push_str(&intro);

        generate_section(&mut str, "Added", added);
        generate_section(&mut str, "Fixed", fixed);
        generate_section(&mut str, "Changed", changed);
        generate_section(&mut str, "Removed", removed);
    }

    for reference in &model.refs {
        let Ref { anchor, href } = reference;
        str.push_str(&format!("[{anchor}]: {href}\n"));
    }

    str
}

fn generate_section(target: &mut String, heading: &str, items: &Vec<String>) {
    if items.is_empty() {
        return;
    }

    target.push_str(&format!("### {heading}\n\n"));
    for item in items {
        let bullet = "- ".to_string()
            + &item
                .lines()
                .fold(String::new(), |a, b| a + "\n  " + b)
                .trim_start()
            + "\n";
        target.push_str(&bullet);
    }
    target.push('\n');
}
