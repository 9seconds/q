#[macro_use]
extern crate clap;


fn main() {
    let _ = clap::App::new("q")
        .author("Sergey Arkhipov <nineseconds@yandex.ru>")
        .version(&crate_version!()[..])
        .about("q is a gentle way to grep using predefined regexp sets.")
        .after_help("Please find more documentation at https://github.com/9seconds/q.")
        .arg(
            clap::Arg::with_name("same_line")
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
            clap::Arg::with_name("RULES")
                .help("Regexp rules to apply to the stdin as a comma-separated list.")
                .index(1)
                .required(true)
        )
        .get_matches();

    println!("Hello world")
}
