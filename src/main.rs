extern crate clap;


fn main() {
    let _ = clap::App::new("q")
        .author("Sergey Arkhipov <nineseconds@yandex.ru>")
        .version("0.1.0")
        .about("q is a gentle way to grep using predefined regexp sets.")
        .after_help("Please find more documentation at https://github.com/9seconds/q.")
        .get_matches();

    println!("Hello world")
}
