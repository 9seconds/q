#[macro_use]
extern crate clap;

extern crate xdg_basedir;
extern crate regex;

use std::path;
use std::io;
use std::io::BufRead;
use std::fs;


fn get_default_config_directory() -> String {
    let mut config_home_dir = xdg_basedir::get_config_home()
        .ok()
        .expect("Unable to detect default q directory.");

    config_home_dir.push("q");
    config_home_dir.push("rules");

    config_home_dir.to_str().unwrap().to_string()
}


fn parse_rules_filenames(rules: &str, config_dir: &str) -> Vec<path::PathBuf> {
    rules
        .split(",")
        .map(
            |item|
            {
                let mut root_path = path::PathBuf::from(config_dir);
                root_path.push(item);
                root_path
            }
        )
        .collect::<Vec<path::PathBuf>>()
}

fn parse_rules(filenames: &Vec<path::PathBuf>) -> Vec<regex::Regex> {
    let mut rules: Vec<regex::Regex> = Vec::with_capacity(filenames.len());

    for filename in filenames.iter() {
        let path = filename.as_path();
        let file = fs::File::open(path)
            .ok()
            .expect(
                &format!("Cannot open file {:?}.", path)
            );

        for line in io::BufReader::new(file).lines() {
            let content = line
                .ok()
                .expect(
                    &format!("Cannot fetch a line from file {:?}.", path)
                );
            let regexp = regex::Regex::new(&content)
                .ok()
                .expect(
                    &format!("Cannot compile regexp from {}, file {:?}.", content, path)
                );
            rules.push(regexp);
        }
    }

    rules
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
            clap::Arg::with_name("RULES_DIRECTORY")
                .help("Directory where rules could be found. By default it uses $XDG_CONFIG_HOME/q/rules")
                .short("-r")
                .long("rules")
                .takes_value(true)
        ).arg(
            clap::Arg::with_name("FILE")
                .help("File to process. Use '-' to read from stdin.")
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

    let default_config_directory = get_default_config_directory();
    let rules_directory = options.value_of("RULES_DIRECTORY").unwrap_or(&default_config_directory);

    let filename = options.value_of("FILE").unwrap_or("-");
    let rules_filenames = parse_rules_filenames(options.value_of("RULES").unwrap(), rules_directory);

    let rules = parse_rules(&rules_filenames);

    println!("Options: {}, {}, {}, {:?}", same_line, rules_directory, filename, rules);
}
