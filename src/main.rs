#[macro_use]
extern crate clap;
extern crate regex;
extern crate xdg_basedir;
extern crate q;

use std::io::{BufRead, Write};
use std::io;
use std::fs;


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
    let rules_directory = q::rules_parser::get_rules_directory(options.value_of("RULES_DIRECTORY"))
        .ok()
        .expect("Cannot determine rules directory!");
    let filename = options
        .value_of("FILE")
        .unwrap_or("-");

    let rules: regex::Regex;
    match q::rules_parser::get_rules(
        &rules_directory, options.value_of("RULES").unwrap(), case_insensitive
    ) {
        Ok(result) => rules = result,
        Err(error) => panic!("Cannot parse rules: {}", error)
    }

    let result = process_filename(&filename, &rules, same_line);
    if let Err(text) = result {
        panic!(text)
    }
}


fn process_filename(filename: &str, rules: &regex::Regex, same_line: bool) -> Result<bool, String> {
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
                let matches = collect_matches(&content, rules);
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
