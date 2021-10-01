#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

static KV_SPLIT: &'static str = "|>!<|";
static LINE_TERM: &'static str = "|<!>|\n";
static LINE_REGEX: &'static str = r"^(.+)\|>!<\|(.+)\|<!>\|$"; 

struct SimpleFileDatabase {
    location: PathBuf,
    data: RefCell<HashMap<String, String>>,
}

impl SimpleFileDatabase {
    pub fn new(location: PathBuf) -> Self {
        let file_content = Self::read_file(&location).unwrap();
        let parsed_data = Self::parse_content(&file_content).unwrap();
        SimpleFileDatabase {
            location,
            data: RefCell::new(parsed_data),
        }
    }

    fn read_file(location: &PathBuf) -> Result<String, String> {
        if location.is_file() {
            Ok(fs::read_to_string(location).unwrap())
        } else if location.exists() {
            Err("Object specified by path exists, but it is not valid file".to_string())
        } else {
            println!("Creating database file at {:?}", location);
            match fs::File::create(location) {
                Ok(_) => Ok(String::new()),
                Err(_) => Err(format!("Not able to create file at {:?}", location)),
            }
        }
    }

    fn parse_content(content: &String) -> Result<HashMap<String, String>, String> {
        let line_regex = Regex::new(LINE_REGEX).unwrap();
        println!("REGEX: {:?}", line_regex);
        if content.is_empty() {
            Ok(HashMap::new())
        } else {
            let lines = content.split_inclusive(LINE_SPLIT);
            lines
                .map(|line| -> Result<(String, String), String> {
                    if !line_regex.is_match(line) {
                        return Err(format!("Line \"{}\" has incompatible format", line))
                    }
                    println!("Line before: {}", line);
                    println!("Line matches? {}", line_regex.is_match(line));
                    let line_parts = line
                        .strip_suffix(LINE_SPLIT)
                        .unwrap()
                        .split(KV_SPLIT)
                        .collect::<Vec<&str>>();
                    // let line_parts = line.split(KV_SPLIT).collect::<Vec<&str>>();
                    // println!("Line parts: {:?}", line_parts);
                    // println!("Line ends with line split: {}", line.ends_with(LINE_SPLIT));
                    if let 2 = line_parts.len() {
                        // println!("Inside len == 2");
                        Ok((line_parts[0].to_string(), line_parts[1].to_string()))
                    } else {
                        Err(format!("Could not parse line \"{}\"", line))
                    }
                })
                .collect()
        }
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
            SimpleFileDatabase::parse_content(&"".to_string()),
            Ok(HashMap::new())
        );
    }

    #[test]
    fn correctly_parses_one_line() {
        let line = format!("hello{}this -is test -value{}", KV_SPLIT, LINE_SPLIT);
        assert_eq!(
            SimpleFileDatabase::parse_content(&line),
            Ok(hashmap!["hello".to_string() => "this -is test -value".to_string()])
        );
    }

    #[test]
    fn fails_for_incorrect_line() {
        let bad_kv_sep = format!("hello{}this -is test -value{}", "<haha>", LINE_SPLIT);
        let bad_line_sep = format!("key{}test-value -here{}", KV_SPLIT, "!line_split!");
        let bad_seps = format!("world{}test-value here<>{}", "|kek|", "!line_split!");

        println!("{:?}", SimpleFileDatabase::parse_content(&bad_kv_sep));
        println!("{:?}", SimpleFileDatabase::parse_content(&bad_line_sep));
        println!("{:?}", SimpleFileDatabase::parse_content(&bad_seps));

        assert!(SimpleFileDatabase::parse_content(&bad_kv_sep).is_err());
        assert!(SimpleFileDatabase::parse_content(&bad_line_sep).is_err());
        assert!(SimpleFileDatabase::parse_content(&bad_seps).is_err());
    }
}
