// App


#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate q;
extern crate pcre;
extern crate xdg_basedir;


struct NormalLogger;
impl log::Log for NormalLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Warn
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }
}


struct DebugLogger;
impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Trace
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!(
                "{level} ({file}:{line}): {message}",
                level=record.level(),
                file=record.location().file(),
                line=record.location().line(),
                message=record.args()
            );
        }
    }
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

    configure_logging(options.is_present("DEBUG"));

    let same_line = options.is_present("SAME_LINE");
    let case_insensitive = options.is_present("CASE_INSENSITIVE");

    let rules_directory = q::rules::get_rules_directory(options.value_of("RULES_DIRECTORY"))
        .ok()
        .expect("Cannot determine rules directory!");
    let filename = options
        .value_of("FILE")
        .unwrap_or("-");

    let rules: pcre::Pcre;
    match q::rules::get_rules(
        &rules_directory, options.value_of("RULES").unwrap(), case_insensitive
    ) {
        Ok(result) => rules = result,
        Err(error) => panic!("Cannot parse rules: {}", error)
    }

    info!("Options: filename={}, rules={:?}, same_line={}", &filename, &rules, same_line);

    let result = q::process::process(&filename, &rules, same_line);
    if let Err(text) = result {
        panic!(text)
    }
}


fn configure_logging(debug: bool) {
    let result = if debug {
        log::set_logger(
            |max_log_level| {
                max_log_level.set(log::LogLevelFilter::Trace);
                Box::new(DebugLogger)
            }
        )
    } else {
        log::set_logger(
            |max_log_level| {
                max_log_level.set(log::LogLevelFilter::Warn);
                Box::new(NormalLogger)
            }
        )
    };

    if let Err(_) = result {
        unreachable!()
    }
}
