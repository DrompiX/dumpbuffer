use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub key: String,
    pub value: String,
}

impl Record {
    pub fn new(key: &String, value: &String) -> Record {
        Record { key: key.to_string(), value: value.to_string() }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n  key: {},\n  value: {}\n}}", self.key, self.value)
    }
}
