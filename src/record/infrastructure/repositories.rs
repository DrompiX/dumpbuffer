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
        InMemoryRecordRepository {
            storage: RefCell::new(HashMap::new()),
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
/// Repository to store records in file in Key-Value format
pub struct KVFileDatabaseRepository {
    /// Underlying file storage implementation
    storage: KVFileDatabase,
}

impl KVFileDatabaseRepository {
    pub fn new(storage: KVFileDatabase) -> Self {
        KVFileDatabaseRepository { storage }
    }
}

impl RecordRepository for KVFileDatabaseRepository {
    fn add(&self, record: Record) -> Result<(), String> {
        self.storage.add(&record.key, &record.value)
    }

    fn get(&self, key: String) -> Result<Record, String> {
        self.storage.get(&key).and_then(|v| Ok(Record::new(&key, &v)))
    }

    fn all(&self) -> Result<Vec<Record>, String> {
        self.storage.items().and_then(|items| -> Result<Vec<Record>, String> {
            Ok(items.iter().map(|(k, v)| Record::new(&k, &v)).collect())
        })
    }
}