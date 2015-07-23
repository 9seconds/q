// Module for stream processing


extern crate pcre;

use std::io;
use std::io::BufRead;
use std::path;

use super::filenames;


pub fn process(filenames: &Vec<path::PathBuf>, rules: &pcre::Pcre, same_line: bool, matches_only: bool) -> bool {
    let mut success = true;

    if filenames.len() == 0 {
        info!("Filename is '-' so use stdin.");

        let stream = io::stdin();
        let mut reader = stream.lock();

        success = process_stream(&mut reader, rules, same_line, matches_only);
    } else {
        for filename in filenames.iter() {
            info!("Filename is '{}' so open a file", filename.display());

            if let Some(file) = filenames::open_if_file(filename.as_path()) {
                let mut reader = io::BufReader::new(file);
                success &= process_stream(&mut reader, rules, same_line, matches_only);
            } else {
                success = false;
            }
        }
    };

    success
}

fn process_stream<R: io::BufRead>(reader: &mut R, rules: &pcre::Pcre, same_line: bool, matches_only: bool) -> bool {
    let mut success = true;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let trimmed_content = content.trim();
                debug!("Line: {}", trimmed_content);

                if matches_only {
                    let matches = collect_matches(&trimmed_content, rules);
                    debug!("Matches: {:?}", matches);

                    if matches.len() == 0 { continue }
                    if same_line {
                        println!("{}", matches.connect(" "));
                    } else {
                        for matched in matches.iter() {
                            println!("{}", matched);
                        }
                    }
                } else {
                    if rules.exec(&trimmed_content).is_some() {
                        println!("{}", &trimmed_content)
                    }
                }
            },
            Err(error) => {
                warn!("{}", error);
                success = false;
            }
        }
    }

    success
}

#[inline]
fn collect_matches(content: &str, rules: &pcre::Pcre) -> Vec<String> {
    rules
        .matches(content)
        .filter_map(|gr| {
            if gr.group_len(0) > 0 {
                Some(gr.group(0).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}
