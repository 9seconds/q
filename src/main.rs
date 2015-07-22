// App


#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate q;

use q::gentle_panic::GentlePanic;


const EX_CANNOT_FIND_RULES_DIRECTORY: i32 = 70;
const EX_CANNOT_PARSE_RULES: i32 = 71;
const EX_CANNOT_PROCESS_STREAM: i32 = 72;


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
            clap::Arg::with_name("DEBUG")
                .help("Run q in debug mode")
                .short("d")
                .long("debug")
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

    q::logging::configure_logging(options.is_present("DEBUG"));

    let same_line = options.is_present("SAME_LINE");
    let case_insensitive = options.is_present("CASE_INSENSITIVE");
    let filename = options
        .value_of("FILE")
        .unwrap_or("-");

    let rules_directory = q::rules::get_rules_directory(options.value_of("RULES_DIRECTORY"))
        .get_or_die_with(EX_CANNOT_FIND_RULES_DIRECTORY, "Cannot discover rules directory!");

    let rules = q::rules::get_rules(
        &rules_directory, options.value_of("RULES").unwrap(), case_insensitive
    )
    .get_or_die_with(EX_CANNOT_PARSE_RULES, "Cannot parse rules");

    info!("Options: filename={}, rules={:?}, same_line={}", &filename, &rules, same_line);

    let _ = q::process::process(&filename, &rules, same_line)
        .get_or_die_with(EX_CANNOT_PROCESS_STREAM, "Cannot process stream");
}
