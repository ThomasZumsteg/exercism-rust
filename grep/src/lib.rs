extern crate failure;

use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    case_sensitive: bool,
    partial: bool,
    list: bool,
    matches: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        return Flags {
            line_numbers: flags.contains(&"-n"),
            case_sensitive: !flags.contains(&"-i"),
            partial: !flags.contains(&"-x"),
            list: flags.contains(&"-l"),
            matches: !flags.contains(&"-v"),
        };
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    for file in files {
        let fh = File::open(file)?;
        for (n, line) in BufReader::new(fh).lines().enumerate() {
            if let Ok(mut text) = line {
                let text_match = match (flags.partial, flags.case_sensitive) {
                    (true, true) => text.contains(pattern),
                    (true, false) => text
                        .to_lowercase()
                        .contains(pattern.to_lowercase().as_str()),
                    (false, true) => text == pattern,
                    (false, false) => text.to_lowercase() == pattern.to_lowercase(),
                };

                if flags.line_numbers { text = format!("{}:{}", n+1, text); }
                if 1 < files.len() { text = format!("{}:{}", file, text); }

                if (text_match && flags.matches) || (!text_match && !flags.matches) {
                    if flags.list {
                        result.push(file.to_string());
                        break;
                    } else { result.push(text); }
                }
            }
        }
    }
    return Ok(result);
}
