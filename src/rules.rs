// Module for rules parsing.


extern crate enum_set;
extern crate pcre;
extern crate xdg_basedir;

use std::collections;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

use super::filenames;


pub fn get_rules_directory(options: Option<&str>) -> Result<path::PathBuf, String> {
    if let Some(directory) = options {
        let dir = path::PathBuf::from(directory);
        info!("Rules directory is defined: {}.", dir.display());
        Ok(dir)
    } else {
        info!("Rules directory is not set, try to discover.");
        let mut config_home_dir = try!(
            xdg_basedir::get_config_home().map_err(|e| e.to_string())
        );
        info!("Config home directory is {}", config_home_dir.display());

        config_home_dir.push("q");
        config_home_dir.push("rules");

        info!("Calculated home directory is {}.", config_home_dir.display());

        Ok(config_home_dir)
    }
}


pub fn get_rules(rules_directory: &path::PathBuf, rules_str: &str, case_insensitive: bool) -> Result<pcre::Pcre, String> {
    let filenames = parse_rules_filenames(rules_str, rules_directory);
    parse_rules(&filenames, case_insensitive)
}


pub fn list(rules_directory: &path::PathBuf) -> Result<Vec<String>, String> {
    let dir_content = try!(fs::read_dir(rules_directory).map_err(|e| e.to_string()));
    let mut content = Vec::<String>::new();

    for entry in dir_content {
        let entry = try!(entry.map_err(|e| e.to_string()));
        let path = entry.path();
        let metadata = try!(fs::metadata(&path).map_err(|e| e.to_string()));
        if metadata.is_dir() {
            continue
        }
        let filename = path.file_name().unwrap().to_os_string().into_string().unwrap();

        if filename == "LICENSE" || filename == "README.md" {
            continue
        }

        content.push(filename);
    }

    content.sort();
    content.dedup();

    Ok(content)
}


#[inline]
fn parse_rules_filenames(rules: &str, config_dir: &path::PathBuf) -> collections::HashSet<path::PathBuf> {
    rules
        .split(",")
        .map(
            |item| {
                let mut root_path = config_dir.clone();
                root_path.push(item);
                debug!("Filepath for {} is {}.", item, root_path.display());
                root_path
            }
        )
        .collect::<collections::HashSet<path::PathBuf>>()
}


fn parse_rules(filenames: &collections::HashSet<path::PathBuf>, case_insensitive: bool) -> Result<pcre::Pcre, String> {
    let mut regex_buffer: Vec<String> = Vec::with_capacity(filenames.len() * 2);

    for filename in filenames.iter() {
        let path = filename.as_path();
        if let Some(file) = filenames::open_if_file(path) {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(content) => {
                        let trimmed_content = content.trim();

                        if trimmed_content == "" {
                            continue
                        }

                        debug!("Add {} to regexp", &trimmed_content);
                        regex_buffer.push(format!("(?:{})", &trimmed_content));
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
        } else {
            return Err(
                format!(
                    "Cannot process file {} with fules", path.display()
                )
            )
        }
    }

    let concatenated_buffer = &regex_buffer.join("|");
    info!("Regexp to compile: {}", concatenated_buffer);

    let mut regex_options: enum_set::EnumSet<pcre::CompileOption> = enum_set::EnumSet::new();
    if case_insensitive {
        regex_options.insert(pcre::CompileOption::Caseless);
    }

    Ok(
        try!(
            pcre::Pcre::compile_with_options(concatenated_buffer, &regex_options).map_err(
                |e| format!("Cannot compile regexps: {}.", e.to_string())
            )
        )
    )
}
