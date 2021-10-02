use crate::record::domain::entities::Record;

pub trait RecordRepository {
    fn add(&self, record: Record) -> Result<(), String>;
    fn get(&self, key: String) -> Result<Record, String>;
    fn all(&self) -> Result<Vec<Record>, String>;
}