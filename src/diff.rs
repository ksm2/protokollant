use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct FileDiff {
    filename: String,
    left: String,
    right: String,
}

impl FileDiff {
    pub fn new(filename: impl Into<String>, left: String, right: String) -> Self {
        let filename = filename.into();
        Self {
            filename,
            left,
            right,
        }
    }
}

struct DiffEntry<T>(usize, usize, usize, usize, Vec<diff::Result<T>>);

pub fn diff_files(file_diffs: &[FileDiff]) {
    for diff in file_diffs {
        diff_file(diff);
    }
}

pub fn diff_file(file_diff: &FileDiff) -> bool {
    let lines = diff::lines(file_diff.left.trim_end(), file_diff.right.trim_end());
    let changed_lines = get_changed_lines(&lines);

    if changed_lines.is_empty() {
        return false;
    }

    let filename = &file_diff.filename;
    println!("{}", format!("--- {filename} before").bold());
    println!("{}", format!("+++ {filename} after").bold());
    let changed_groups = group_changes(changed_lines, lines.len());
    let changed_groups = with_diff(changed_groups, lines);

    for DiffEntry(from1, count1, from2, count2, lines) in changed_groups {
        let string = format!("@@ -{from1},{count1} +{from2},{count2} @@").cyan();
        println!("{}", string);
        for line in lines {
            match line {
                diff::Result::Left(l) => println!("{}{}", "-".red(), l.red()),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("{}{}", "+".green(), r.green()),
            }
        }
    }

    true
}

fn get_changed_lines(lines: &[diff::Result<&str>]) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter(|(_, diff)| !matches!(diff, diff::Result::Both(_, _)))
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

fn group_changes(line_numbers: Vec<usize>, len: usize) -> Vec<(usize, usize)> {
    let mut groups = Vec::new();
    if let (Some(&first), Some(&last)) = (line_numbers.first(), line_numbers.last()) {
        let mut filter_pairs = line_numbers
            .iter()
            .copied()
            .tuple_windows()
            .filter(|(a, b)| b - a > 7)
            .collect::<Vec<_>>();
        filter_pairs.insert(0, (0, first));
        filter_pairs.push((last, 0));

        for ((_, a), (b, _)) in filter_pairs.into_iter().tuple_windows() {
            groups.push((a.max(3) - 3, b.min(len - 4) + 3));
        }
    }

    groups
}

fn with_diff(groups: Vec<(usize, usize)>, lines: Vec<diff::Result<&str>>) -> Vec<DiffEntry<&str>> {
    let mut result = Vec::new();

    let mut left = 0;
    let mut right = 0;

    for (from, to) in groups {
        let mut vec = Vec::new();

        let from1 = from - right + 1;
        let from2 = from - left + 1;

        for index in from..=to {
            let line = lines.get(index).unwrap();
            match line {
                diff::Result::Left(_) => {
                    left += 1;
                }
                diff::Result::Right(_) => {
                    right += 1;
                }
                _ => {}
            }
            vec.push(line.clone());
        }

        let count1 = to - right - from1 + 2;
        let count2 = to - left - from2 + 2;

        result.push(DiffEntry(from1, count1, from2, count2, vec));
    }
    result
}
