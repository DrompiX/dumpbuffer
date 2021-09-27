use std::fmt::Display;

#[derive(Debug)]
pub struct Record<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Display for Record<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ key: {}, value: {} }}", self.key, self.value)
    }
}
