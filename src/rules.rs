// Module for rules_parser


extern crate regex;
extern crate xdg_basedir;

use std::collections;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;


pub fn get_rules_directory(options: Option<&str>) -> Result<path::PathBuf, String> {
    if let Some(directory) = options {
        Ok(path::PathBuf::from(directory))
    } else {
        let mut config_home_dir = try!(
            xdg_basedir::get_config_home().map_err(|e| e.to_string())
        );

        config_home_dir.push("q");
        config_home_dir.push("rules");

        Ok(config_home_dir)
    }
}


pub fn get_rules(rules_directory: &path::PathBuf, rules_str: &str, case_insensitive: bool) -> Result<regex::Regex, String> {
    let filenames = parse_rules_filenames(rules_str, rules_directory);
    parse_rules(&filenames, case_insensitive)
}


fn parse_rules_filenames(rules: &str, config_dir: &path::PathBuf) -> collections::HashSet<path::PathBuf> {
    rules
        .split(",")
        .map(
            |item| {
                let mut root_path = config_dir.clone();
                root_path.push(item);
                root_path
            }
        )
        .collect::<collections::HashSet<path::PathBuf>>()
}


fn parse_rules(filenames: &collections::HashSet<path::PathBuf>, case_insensitive: bool) -> Result<regex::Regex, String> {
    let mut regex_buffer: Vec<String> = Vec::with_capacity(filenames.len() * 2);

    for filename in filenames.iter() {
        let path = filename.as_path();
        let file = try!(fs::File::open(path).map_err(|e| e.to_string()));
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let trimmed_content = content.trim_right();
                    regex_buffer.push(
                        if case_insensitive {
                            format!("(?i:{})", &trimmed_content)
                        } else {
                            format!("({})", &trimmed_content)
                        }
                    )
                },
                Err(error) => return Err(
                    format!(
                        "Cannot fetch a line from file {}: {}!",
                        path.display(),
                        error.to_string()
                    )
                )
            }
        }
    }

    Ok(
        try!(
            regex::Regex::new(&regex_buffer.connect("|")).map_err(
                |e| format!("Cannot compile regexps: {}.", e.to_string())
            )
        )
    )
}
