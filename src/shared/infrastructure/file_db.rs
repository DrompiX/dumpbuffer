#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fs, fs::File, path::PathBuf};

static KV_SPLIT: &'static str = "|>!<|\n";
static LINE_SPLIT: &'static str = "|<!>|\n";

struct SimpleFileDatabase {
    location: PathBuf,
    data: RefCell<HashMap<String, String>>,
}

impl SimpleFileDatabase {
    pub fn new(location: PathBuf) -> Self {
        let file_content = Self::read_file_content(&location);
        SimpleFileDatabase {
            location,
            data: RefCell::new(HashMap::new()),
        }
    }

    fn read_file_content(location: &PathBuf) -> HashMap<String, String> {
        if location.is_file() {
            let content = fs::read_to_string(location).unwrap();
            content.split(LINE_SPLIT).map(|line| -> (String, String) {
                let line_parts = line.split(KV_SPLIT).collect::<Vec<&str>>();
                if let 2 = line_parts.len() {
                    (line_parts[0].to_string(), line_parts[1].to_string())
                } else {
                    // TODO: add better exception message with line output
                    panic!("Can not parse file")
                }
            }).collect::<HashMap<String, String>>()
        
        } else {
            File::create(location)
                .expect(format!("Not able to create file at {:?}", location).as_str());
            HashMap::new()
        }
    }
}
