use crate::record::domain::entities::Record;

pub trait RecordRepository {
    /// Add new record to the data storage
    fn add(&self, record: Record) -> Result<(), String>;
    /// Get record by key
    fn get(&self, key: String) -> Result<Record, String>;
    /// Get all records from the data storage
    fn all(&self) -> Result<Vec<Record>, String>;
}

impl<T> RecordRepository for Box<T> where T: RecordRepository + ?Sized {
    fn add(&self, record: Record) -> Result<(), String> {
        (**self).add(record)
    }

    fn get(&self, key: String) -> Result<Record, String> {
        (**self).get(key)
    }

    fn all(&self) -> Result<Vec<Record>, String> {
        (**self).all()
    }
}