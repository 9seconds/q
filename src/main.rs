#[macro_use]
extern crate clap;
extern crate regex;
extern crate xdg_basedir;

use std::collections;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::path::PathBuf;


fn get_rules_directory(options: Option<&str>) -> Result<PathBuf, String> {
    if let Some(directory) = options {
        Ok(PathBuf::from(directory))
    } else {
        let mut config_home_dir = try!(
            xdg_basedir::get_config_home().map_err(|e| e.to_string())
        );

        config_home_dir.push("q");
        config_home_dir.push("rules");

        Ok(config_home_dir)
    }
}


fn parse_rules_filenames(rules: &str, config_dir: &PathBuf) -> collections::HashSet<path::PathBuf> {
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
                    let quoted_content = &regex::quote(&content);
                    regex_buffer.push(
                        if case_insensitive {
                            format!("(?i{})", quoted_content)
                        } else {
                            format!("({})", quoted_content)
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

    let compiled_result = regex::Regex::new(&regex_buffer.connect("|"));
    if let Err(e) = compiled_result {
        return Err(format!("Cannot compile regexps: {}", e.to_string()))
    }

    Ok(compiled_result.ok().unwrap())
}


fn get_rules(rules_directory: &PathBuf, rules_str: &str, case_insensitive: bool) -> Result<regex::Regex, String> {
    let filenames = parse_rules_filenames(rules_str, rules_directory);
    parse_rules(&filenames, case_insensitive)
}



fn main() {
    let options = clap::App::new("q")
        .author("Sergey Arkhipov <nineseconds@yandex.ru>")
        .version(&crate_version!()[..])
        .about("q is a gentle way to grep using predefined regexp sets.")
        .after_help("Please find more documentation at https://github.com/9seconds/q.")
        .arg(
            clap::Arg::with_name("SAME_LINE")
                .help("Keep matches on the same line")
                .short("l")
                .long("same_line")
        ).arg(
            clap::Arg::with_name("CASE_INSENSITIVE")
                .help("Use case insensitive regex versions.")
                .short("i")
                .long("case-insensitive")
        ).arg(
            clap::Arg::with_name("RULES_DIRECTORY")
                .help("Directory where rules could be found. By default it uses $XDG_CONFIG_HOME/q/rules")
                .short("-r")
                .long("rules")
                .takes_value(true)
        ).arg(
            clap::Arg::with_name("FILE")
                .help("File to process. Use '-' to read from stdin (default is stdin).")
                .short("-f")
                .long("file")
                .takes_value(true)
        ).arg(
            clap::Arg::with_name("RULES")
                .help("Regexp rules to apply to the stdin as a comma-separated list.")
                .index(1)
                .required(true)
        )
        .get_matches();

    let same_line = options.is_present("SAME_LINE");
    let case_insensitive = options.is_present("CASE_INSENSITIVE");
    let rules_directory = get_rules_directory(options.value_of("RULES_DIRECTORY"))
        .ok()
        .expect("Cannot determine rules directory!");
    let filename = options
        .value_of("FILE")
        .unwrap_or("-");

    let rules: regex::Regex;
    match get_rules(&rules_directory, options.value_of("RULES").unwrap(), case_insensitive) {
        Ok(result) => rules = result,
        Err(error) => panic!("Cannot parse rules: {}", error)
    }

    println!("Options: {}, {}, {}, {:?}", same_line, case_insensitive, filename, rules);
}
