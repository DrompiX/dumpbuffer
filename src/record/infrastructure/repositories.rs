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
    fn add(&self, record: Record) {
        self.storage.borrow_mut().insert(record.key.to_string(), record.value.to_string());
    }

    fn get(&self, key: String) -> Result<Record, String> {
        match self.storage.borrow().get(&key).clone() {
            Some(v) => Ok(Record { key, value: v.to_string() }),
            None => Err(format!("Record with key \"{}\" does not exist", key))
        }
    }

    fn all(&self) -> Vec<Record> {
        self.storage.borrow().iter()
            .map(|(k, v)| Record::new(k, v))
            .collect()
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
    fn add(&self, record: Record) {
        self.storage.add(&record.key, &record.value).unwrap();
    }

    fn get(&self, key: String) -> Result<Record, String> {
        todo!()
    }

    fn all(&self) -> Vec<Record> {
        todo!()
    }
}