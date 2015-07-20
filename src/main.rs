#[macro_use]
extern crate clap;
extern crate regex;
extern crate xdg_basedir;
extern crate q;


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

    println!("Options: {}, {}, {}, {:?}", same_line, case_insensitive, filename, rules);
}
