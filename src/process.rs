// Module for stream processing


extern crate pcre;

use std::borrow;
use std::borrow::Borrow;
use std::io;
use std::io::BufRead;
use std::path;

use super::filenames;


static PATTERNS_TO_TRIM: &'static [char] = &['\r', '\n'];


struct Printer<'f, 'r> {
    filename: &'f str,
    regex: &'r pcre::Pcre,
    same_line: bool,
    matches_only: bool,
    line_numbers: bool
}

impl<'f, 'r> Printer<'f, 'r> {
    fn new(filename: &'f str, regex: &'r pcre::Pcre, same_line: bool, matches_only: bool, line_numbers: bool) -> Printer<'f, 'r> {
        Printer{
            filename: filename,
            regex: regex,
            same_line: same_line,
            matches_only: matches_only,
            line_numbers: line_numbers
        }
    }

    #[inline]
    fn get_line_prefix(&self, line_number: usize) -> borrow::Cow<'static, str> {
        if self.line_numbers {
            if self.filename != "" {
                format!("{}:{}\t", self.filename, line_number).into()
            } else {
                format!("{}\t", line_number).into()
            }
        } else {
            "".into()
        }
    }

    #[inline]
    fn print_line(&self, content: &str, line_number: usize) {
        println!("{}{}", self.get_line_prefix(line_number), content)
    }

    #[inline]
    fn print(&self, content: &str, line_number: usize) {
        if self.matches_only {
            let matches = self.collect_matches(content);
            debug!("Matches for {} are {:?}", content, &matches);
            self.show_matches(&matches, line_number)
        } else {
            if self.regex.exec(&content).is_some() {
                self.print_line(content, line_number)
            }
        }
    }

    #[inline]
    fn show_matches(&self, matches: &Vec<String>, line_number: usize) {
        if matches.len() > 0 {
            if self.same_line {
                self.print_line(&matches.join(" "), line_number)
            } else {
                for item in matches.iter() {
                    self.print_line(&item, line_number)
                }
            }
        }
    }

    #[inline]
    fn collect_matches(&self, content: &str) -> Vec<String> {
        self.regex
            .matches(content)
            .map(|gr| gr.group(0).to_string())
            .collect::<Vec<String>>()
    }
}


pub fn process(filenames: &Vec<path::PathBuf>, rules: &pcre::Pcre, same_line: bool, matches_only: bool, line_numbers: bool) -> bool {
    let mut success = true;

    if filenames.len() == 0 {
        info!("Filename is '-' so use stdin.");

        let stream = io::stdin();
        let mut reader = stream.lock();
        let printer = Printer::new("", rules, same_line, matches_only, line_numbers);

        success = process_stream(&mut reader, &printer);
    } else {
        for filename in filenames.iter() {
            info!("Filename is '{}' so open a file", filename.display());

            if let Some(file) = filenames::open_if_file(filename.as_path()) {
                let mut reader = io::BufReader::new(file);
                let printable_name = filename.to_string_lossy();
                let printer = Printer::new(printable_name.borrow(), rules, same_line, matches_only, line_numbers);

                success &= process_stream(&mut reader, &printer);
            } else {
                success = false;
            }
        }
    };

    success
}


fn process_stream<R: io::BufRead>(reader: &mut R, printer: &Printer) -> bool {
    let mut success = true;

    for (line_number, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let trimmed_content = content.trim_right_matches(PATTERNS_TO_TRIM);
                debug!("Line: {}", trimmed_content);
                printer.print(&trimmed_content, line_number + 1);
            },
            Err(error) => {
                warn!("{}", error);
                success = false;
            }
        }
    }

    success
}
