use std::cell::RefCell;
use std::collections::HashMap;

use crate::record::domain::entities::Record;
use crate::record::domain::repositories::RecordRepository;

pub struct InMemoryRepository {
    storage: RefCell<HashMap<String, String>>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        let mut hm = HashMap::new();
        hm.insert("hello".to_string(), "world".to_string());
        hm.insert("hey".to_string(), "how are you doing? ˚∆∆˚".to_string());
        InMemoryRepository {
            storage: RefCell::new(hm),
        }
    }
}

impl RecordRepository for InMemoryRepository {
    fn add(&self, record: Record) {
        self.storage.borrow_mut().insert(record.key.to_string(), record.value.to_string());
    }

    fn get(&self, key: String) -> Result<Record, String> {
        match self.storage.borrow().get(&key).clone() {
            Some(v) => Ok(Record { key, value: v.to_string() }),
            None => Err(String::from(format!("Record with key \"{}\" does not exist", key)))
        }
    }

    fn all(&self) -> Vec<Record> {
        self.storage.borrow().iter()
            .map(|(k, v)| Record::new(k, v))
            .collect()
    }
}
