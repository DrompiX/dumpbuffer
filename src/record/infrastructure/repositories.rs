#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

use crate::record::domain::entities::Record;
use crate::record::domain::repositories::RecordRepository;
use crate::shared::infrastructure::file_db::KVFileDatabase;

/// Repository to store records in memory. Primary use case - testing
pub struct InMemoryRecordRepository {
    /// HashMap-based key-value storage for records
    storage: RefCell<HashMap<String, String>>,
}

impl InMemoryRecordRepository {
    pub fn new() -> InMemoryRecordRepository {
        let mut hm = HashMap::new();
        hm.insert("hello".to_string(), "world".to_string());
        hm.insert("hey".to_string(), "how are you doing? ˚∆∆˚".to_string());
        InMemoryRecordRepository {
            storage: RefCell::new(hm),
        }
    }
}

impl RecordRepository for InMemoryRecordRepository {
    fn add(&self, record: Record) -> Result<(), String> {
        let (key, value) = (record.key.to_string(), record.value.to_string());
        match self.storage.borrow_mut().insert(key.to_string(), value) {
            Some(_) => Err(format!("Value for key \"{}\" was updated", key)),
            None => Ok(())
        }
    }

    fn get(&self, key: String) -> Result<Record, String> {
        match self.storage.borrow().get(&key).clone() {
            Some(v) => Ok(Record { key, value: v.to_string() }),
            None => Err(format!("Record with key \"{}\" does not exist", key))
        }
    }

    fn all(&self) -> Result<Vec<Record>, String> {
        Ok(self.storage.borrow().iter()
            .map(|(k, v)| Record::new(k, v))
            .collect())
    }
}

////////////////////////////////////////////
pub struct KVFileDatabaseRepository<'a> {
    storage: &'a KVFileDatabase,
}

impl<'a> KVFileDatabaseRepository<'a> {
    pub fn new(storage: &'a KVFileDatabase) -> Self {
        KVFileDatabaseRepository { storage }
    }
}

impl<'a> RecordRepository for KVFileDatabaseRepository<'a> {
    fn add(&self, record: Record) -> Result<(), String> {
        self.storage.add(&record.key, &record.value)
    }

    fn get(&self, key: String) -> Result<Record, String> {
        self.storage.get(&key).and_then(|v| Ok(Record::new(&key, &v)))
    }

    fn all(&self) -> Result<Vec<Record>, String> {
        self.storage.items().and_then(|items: Vec<(String, String)>| -> Result<Vec<Record>, String> {
            Ok(items.iter().map(|(k, v)| Record::new(&k, &v)).collect())
        })
    }
}