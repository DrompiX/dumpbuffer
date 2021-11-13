use std::ffi::OsString;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, PartialEq)]
pub enum DumpBufferCLI {
    #[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    /// Add new record to the storage
    Add {
        key: String,
        #[structopt(required = true, parse(from_os_str))]
        value: Vec<OsString>,
    },
    /// Get record with specific key
    Get {
        key: String,
    },
    /// Execute record with specific key
    Exec {
        key: String,
    },
    /// List all available records
    List {
        #[structopt(long)]
        keys_only: bool,
    },
    /// Delete record from storage by key or all records at once
    #[structopt(name = "rm")]
    Delete {
        #[structopt(required_unless_one(&["all"]))]
        /// Delete record by key
        key: Option<String>,
        #[structopt(long, conflicts_with = "key")]
        /// Delete all records
        all: bool,
    }
}

impl DumpBufferCLI {
    #[allow(dead_code)]
    pub fn joined_value(&self, separator: &str) -> Option<String> {
        match self {
            DumpBufferCLI::Add { key: _, value } => {
                let vals: Vec<String> = value
                    .iter()
                    .map(|v| {
                        let result_string = v.to_owned().into_string();
                        result_string.expect(format!("Cant parse OsString value {:?}", v).as_str())
                    })
                    .collect();
                Some(vals.join(separator))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_works_with_key_value() {
        assert_eq!(
            DumpBufferCLI::from_iter(&["test", "add", "some-key", "some-value"]),
            DumpBufferCLI::Add {
                key: "some-key".to_string(),
                value: [OsString::from("some-value")].to_vec()
            }
        );
    }

    #[test]
    fn add_works_when_value_has_flags() {
        assert_eq!(
            DumpBufferCLI::from_iter(&["test", "add", "key", "value", "-c", "hey"]),
            DumpBufferCLI::Add {
                key: "key".to_string(),
                value: [
                    OsString::from("value"),
                    OsString::from("-c"),
                    OsString::from("hey")
                ]
                .to_vec()
            }
        );
    }

    #[test]
    fn get_works_with_key() {
        assert_eq!(
            DumpBufferCLI::from_iter(&["test", "get", "key-name"]),
            DumpBufferCLI::Get {
                key: "key-name".to_string()
            }
        );
    }

    #[test]
    fn list_is_parsed_correctly_without_flag() {
        assert_eq!(
            DumpBufferCLI::from_iter(&["test", "list"]),
            DumpBufferCLI::List { keys_only: false }
        );
    }

    #[test]
    fn list_is_parsed_correctly_with_flag() {
        assert_eq!(
            DumpBufferCLI::from_iter(&["test", "list", "--keys-only"]),
            DumpBufferCLI::List { keys_only: true }
        );
    }

    #[test]
    fn joined_value_works_correctly() {
        let v = DumpBufferCLI::from_iter(&["test", "add", "key", "bash", "-c", "hey"]);
        assert_eq!(v.joined_value(" "), Some("bash -c hey".to_string()));
    }

    #[test]
    fn joined_value_accepts_different_separators() {
        let v = DumpBufferCLI::from_iter(&["test", "add", "key", "bash", "-c", "hey"]);
        assert_eq!(v.joined_value("|"), Some("bash|-c|hey".to_string()));
        assert_eq!(v.joined_value("?"), Some("bash?-c?hey".to_string()));
        assert_eq!(v.joined_value("*"), Some("bash*-c*hey".to_string()));
    }

    #[test]
    fn joined_value_is_none_for_other_commands() {
        let get_cmd = DumpBufferCLI::Get {
            key: "any".to_string(),
        };
        let list_cmd = DumpBufferCLI::List { keys_only: false };
        assert_eq!(get_cmd.joined_value(" "), None);
        assert_eq!(list_cmd.joined_value(" "), None);
    }
}
