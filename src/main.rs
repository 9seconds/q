// App


#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate q;

use std::process;

use q::gentle_panic::GentlePanic;


const EX_OK: i32 = 0;
const EX_CANNOT_FIND_RULES_DIRECTORY: i32 = 70;
const EX_CANNOT_PROCESS_GLOB_PATTERNS: i32 = 71;
const EX_CANNOT_PARSE_RULES: i32 = 72;
const EX_CANNOT_PROCESS_STREAM: i32 = 73;
const EX_CANNOT_LIST_DIRECTORY: i32 = 74;
const EX_PLEASE_DEFINE_ALL_ATTRIBUTES: i32 = 75;


fn main() {
    let options = clap::App::new("q")
        .author("Sergey Arkhipov <nineseconds@yandex.ru>")
        .version(&crate_version!()[..])
        .about("q is a gentle way to grep using predefined regexp sets.")
        .after_help("Please find more documentation at https://github.com/9seconds/q or using man page.")
        .arg(
            clap::Arg::with_name("SAME_LINE")
                .help("Keep matches on the same line")
                .short("s")
                .long("same-line")
        ).arg(
            clap::Arg::with_name("LINE_NUMBERS")
                .help("Print line numbers and filenames")
                .short("n")
                .long("line-numbers")
        ).arg(
            clap::Arg::with_name("MATCHES_ONLY")
                .help("Print matches only, not whole line")
                .short("o")
                .long("matches-only")
        ).arg(
            clap::Arg::with_name("DEBUG")
                .help("Run q in debug mode")
                .short("d")
                .long("debug")
        ).arg(
            clap::Arg::with_name("CASE_INSENSITIVE")
                .help("Use case insensitive regex versions")
                .short("i")
                .long("case-insensitive")
        ).arg(
            clap::Arg::with_name("LIST_REGEXPS")
                .help("Just list regular expressions, nothing else")
                .short("l")
                .long("list")
        ).arg(
            clap::Arg::with_name("RULES_DIRECTORY")
                .help("Directory where rules could be found. By default it uses $XDG_CONFIG_HOME/q/rules")
                .short("-r")
                .long("rules")
                .takes_value(true)
        ).arg(
            clap::Arg::with_name("RULES")
                .help("Regexp rules to apply to the stdin as a comma-separated list")
                .index(1)
        ).arg(
            clap::Arg::with_name("FILES")
                .help("Files to process. If no file is specified then q will consume stdin")
                .index(2)
                .multiple(true)
        )
        .get_matches();

    q::logging::configure_logging(options.is_present("DEBUG"));

    let same_line = options.is_present("SAME_LINE");
    let case_insensitive = options.is_present("CASE_INSENSITIVE");
    let matches_only = options.is_present("MATCHES_ONLY");
    let line_numbers = options.is_present("LINE_NUMBERS");
    let list_only = options.is_present("LIST_REGEXPS");

    let rules_directory = q::rules::get_rules_directory(options.value_of("RULES_DIRECTORY"))
        .get_or_die_with(EX_CANNOT_FIND_RULES_DIRECTORY, "Cannot discover rules directory!");
    if list_only {
        let to_list = q::rules::list(&rules_directory)
            .get_or_die_with(EX_CANNOT_LIST_DIRECTORY, "Cannot list rules directory!");
        for item in to_list.iter() {
            println!("{}", &item)
        }
        process::exit(EX_OK);
    }

    let rules_set = options.value_of("RULES")
        .get_or_die_with(EX_PLEASE_DEFINE_ALL_ATTRIBUTES, "Please define rules option.");
    let rules = q::rules::get_rules(&rules_directory, &rules_set, case_insensitive)
        .get_or_die_with(EX_CANNOT_PARSE_RULES, "Cannot parse rules");
    let filenames = q::filenames::extract(options.values_of("FILES"))
        .get_or_die_with(EX_CANNOT_PROCESS_GLOB_PATTERNS, "Some problems with glob patters you set, please check");

    info!("Options: filenames={:?}, rules={:?}, same_line={}, matches_only={}, line_numbers={}",
          &filenames, &rules, same_line, matches_only, line_numbers);

    if !q::process::process(&filenames, &rules, same_line, matches_only, line_numbers) {
        process::exit(EX_CANNOT_PROCESS_STREAM)
    }
}
