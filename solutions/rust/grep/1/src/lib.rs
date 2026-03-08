use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use anyhow::{Error, Ok};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags {
    line_numbers: bool,
    output_filenames_only: bool,
    case_insensitive: bool,
    invert_search: bool,
    match_entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut out = Flags::default();
        for flag in flags {
            match *flag {
                "-n" => out.line_numbers = true,
                "-l" => out.output_filenames_only = true,
                "-i" => out.case_insensitive = true,
                "-v" => out.invert_search = true,
                "-x" => out.match_entire_line = true,
                _ => (),
            }
        }
        out
    }
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let haystack_chars = haystack
        .chars()
        .map(|c| c.to_lowercase())
        .flatten()
        .collect::<Vec<_>>();
    let needle_chars = needle
        .chars()
        .map(|c| c.to_lowercase())
        .flatten()
        .collect::<Vec<_>>();

    if needle_chars.len() > haystack_chars.len() {
        return false;
    }

    for i in 0..=(haystack_chars.len() - needle_chars.len()) {
        if haystack_chars[i..(i + needle_chars.len())].eq(&needle_chars) {
            return true;
        }
    }

    false
}

fn does_match(pattern: &str, flags: &Flags, line: &str) -> bool {
    let result = if flags.case_insensitive {
        if flags.match_entire_line {
            line.eq_ignore_ascii_case(pattern)
        } else {
            contains_ignore_ascii_case(line, pattern)
        }
    } else {
        if flags.match_entire_line {
            line.eq(pattern)
        } else {
            line.contains(pattern)
        }
    };

    if flags.invert_search {
        !result
    } else {
        result
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let multi_files = files.len() > 1;

    for filename in files {
        let lines = read_lines(filename)?;
        for (i, line) in lines.enumerate() {
            let line = line?;
            dbg!("{line:?}");
            if does_match(pattern, flags, &line) {
                if flags.output_filenames_only {
                    results.push(filename.to_string());
                    break; // only add filename once
                }

                let mut final_line = String::new();
                if multi_files {
                    final_line.push_str(&filename);
                    final_line.push(':');
                }
                if flags.line_numbers {
                    final_line.push_str(&(i + 1).to_string());
                    final_line.push(':');
                }
                final_line.push_str(&line);

                results.push(final_line);
            }
        }
    }
    Ok(results)
}
