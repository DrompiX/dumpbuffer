use crate::record::domain::entities::Record;

pub trait RecordRepository {
    fn add(&self, record: Record);
    fn get(&self, key: String) -> Record;
    fn all(&self) -> Vec<Record>;
}