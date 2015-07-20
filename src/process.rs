// Module for stream processing


extern crate regex;

use std::fs;
use std::io;
use std::io::{BufRead, Write};



pub fn process(filename: &str, rules: &regex::Regex, same_line: bool) -> Result<bool, String> {
    if filename == "-" {
        let stream = io::stdin();
        let mut reader = stream.lock();
        process_stream(&mut reader, rules, same_line)
    } else {
        let file = try!(fs::File::open(filename).map_err(|e| e.to_string()));
        let mut reader = io::BufReader::new(file);
        process_stream(&mut reader, rules, same_line)
    }
}

fn process_stream<R: io::BufRead>(reader: &mut R, rules: &regex::Regex, same_line: bool) -> Result<bool, String> {
    let stdout_stream = io::stdout();
    let mut stdout = stdout_stream.lock();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let trimmed_content = content.trim();

                let matches = collect_matches(&trimmed_content, rules);
                if matches.len() == 0 {
                    continue
                }
                if same_line {
                    println!("{}", matches.connect(" "));
                    let _ = stdout.flush();
                } else {
                    for matched in matches.iter() {
                        println!("{}", matched);
                        let _ = stdout.flush();
                    }
                }
            },
            Err(error) => return Err(error.to_string())
        }
    }

    Ok(true)
}

fn collect_matches(content: &str, rules: &regex::Regex) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    for group in rules.captures_iter(content) {
        if let Some(text) = group.at(0) {
            matches.push(text.to_string())
        }
    }

    matches
}
