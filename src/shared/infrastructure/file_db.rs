#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

static KV_SPLIT: &'static str = "|>!<|";
static LINE_TERM: &'static str = "|<!>|\n";
static LINE_REGEX: &'static str = r"^(.+)\|>!<\|(.+)\|<!>\|\n$"; 

pub struct KVFileDatabase {
    location: PathBuf,
    data: RefCell<HashMap<String, String>>,
}

impl KVFileDatabase {
    pub fn new(location: &PathBuf) -> Result<Self, String> {
        let file_content = Self::read_file(&location)?;
        let parsed_data = Self::parse_content(&file_content)?;
        Ok(KVFileDatabase {
            location: location.clone(),
            data: RefCell::new(parsed_data),
        })
    }

    pub fn add(&self, key: &String, value: &String) -> Result<(), String> {
        let mut storage = self.data.borrow_mut();
        if storage.contains_key(key) {
            Err(format!("Key \"{}\" already exists", key))
        } else {
            storage.insert(key.to_string(), value.to_string());
            Ok(())
        }
    }

    pub fn get(&self, key: &String) -> Result<String, String> {
        match self.data.borrow().get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err(format!("Key \"{}\" does not exist", key)),
        }
    }

    pub fn items(&self) -> Result<Vec<(String, String)>, String> {
        Ok(self.data
            .borrow()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
        )
    }

    fn construct_dump(&self) -> String {
        self.data
            .borrow()
            .iter()
            .map(|(k, v)| format!("{}{}{}{}", k, KV_SPLIT, v, LINE_TERM))
            .collect::<Vec<String>>()
            .join("")
    }

    fn read_file(location: &PathBuf) -> Result<String, String> {
        if location.is_file() {
            // DB file already exists, reading its content
            Ok(fs::read_to_string(location).unwrap())
        } else if location.exists() {
            // Location specifies not valid file (maybe it is a directory)
            Err("Object specified by path exists, but it is not a valid file".to_string())
        } else {
            // File does not exist yet, consider as empty
            Ok(String::new())
        }
    }

    fn parse_content(content: &String) -> Result<HashMap<String, String>, String> {
        let line_regex = Regex::new(LINE_REGEX).unwrap();
        let lines = content.split_inclusive(LINE_TERM);
        lines.map(|line| -> Result<(String, String), String> {
            match line_regex.captures(line) {
                Some(groups) if groups.len() == 3 => {
                    Ok((groups[1].to_string(), groups[2].to_string()))
                },
                _ => Err(format!("Line \"{}\" has incompatible format", line)),
            }
        }).collect()
    }
}

impl Drop for KVFileDatabase {
    /// Save updated hashmap to file storage
    fn drop(&mut self) {
        fs::write(&self.location, self.construct_dump()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    #[test]
    fn content_parsed_correctly_for_empty() {
        assert_eq!(
            KVFileDatabase::parse_content(&"".to_string()),
            Ok(HashMap::new())
        );
    }

    #[test]
    fn correctly_parses_one_line() {
        let line = format!("hello{}this -is test -value{}", KV_SPLIT, LINE_TERM);
        assert_eq!(
            KVFileDatabase::parse_content(&line),
            Ok(hashmap!["hello".to_string() => "this -is test -value".to_string()])
        );
    }

    #[test]
    fn fails_for_incorrect_line() {
        let bad_kv_sep = format!("hello{}this -is test -value{}", "<haha>", LINE_TERM);
        let bad_line_sep = format!("key{}test-value -here{}", KV_SPLIT, "!line_split!");
        let bad_seps = format!("world{}test-value here<>{}", "|kek|", "!line_split!");

        println!("{:?}", KVFileDatabase::parse_content(&bad_kv_sep));
        println!("{:?}", KVFileDatabase::parse_content(&bad_line_sep));
        println!("{:?}", KVFileDatabase::parse_content(&bad_seps));

        assert!(KVFileDatabase::parse_content(&bad_kv_sep).is_err());
        assert!(KVFileDatabase::parse_content(&bad_line_sep).is_err());
        assert!(KVFileDatabase::parse_content(&bad_seps).is_err());
    }
}
