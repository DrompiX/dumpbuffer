use crate::record::domain::entities::Record;

pub trait RecordRepository {
    fn add(&self, record: Record);
    fn get(&self, key: String) -> Result<Record, String>;
    fn all(&self) -> Vec<Record>;
}