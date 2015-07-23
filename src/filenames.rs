// Module for extracting filenames


extern crate glob;

use std::fs;
use std::collections;
use std::path;


pub fn extract(values: Option<Vec<&str>>) -> Result<Vec<path::PathBuf>, String> {
    let mut result = Vec::<path::PathBuf>::new();
    let mut already_seen = collections::HashSet::<String>::new();

    if let Some(patterns) = values {
        for pattern in patterns.iter() {
            let mut found = false;

            for filename in glob::glob(pattern).unwrap() {
                found = true;
                match filename {
                    Ok(fname) => {
                        let lossy_name = fname.to_string_lossy().into_owned();
                        if !already_seen.contains(&lossy_name) {
                            already_seen.insert(lossy_name);
                            result.push(fname);
                        }
                    },
                    Err(error) => return Err(error.to_string())
                }
            }
            if !found {
                return Err(format!("Cannot find anything by pattern {}", &pattern))
            }
        }
    }

    Ok(result)
}

pub fn open_if_file(path: &path::Path) -> Option<fs::File> {
    match fs::metadata(path).map_err(|e| e.to_string()) {
        Ok(data) => {
            if data.is_file() {
                let file = fs::File::open(path).map_err(|e| e.to_string());
                match file {
                    Ok(file_handler) => {
                        return Some(file_handler)
                    },
                    Err(err) => {
                        error!("Cannot open a file {}: {}.", path.display(), err);
                    }
                }
            } else {
                error!("Path {} is not a file!", path.display())
            }
        },
        Err(err) => {
            error!("Cannot fetch metadata of the file {}: {}", path.display(), err)
        }
    }

    None
}
