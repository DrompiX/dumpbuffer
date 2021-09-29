#![allow(dead_code)]

use super::queries::{AddNewRecordQuery, GetRecordQuery, ListRecordsQuery};
use crate::record::domain::{entities::Record, repositories::RecordRepository};

pub struct AddNewRecordService<'a> {
    record_repository: &'a dyn RecordRepository,
}

impl<'a> AddNewRecordService<'a> {
    pub fn new(record_repository: &dyn RecordRepository) -> AddNewRecordService {
        return AddNewRecordService { record_repository };
    }

    pub fn run(&self, query: AddNewRecordQuery) {
        let record = Record {
            key: query.key,
            value: query.value,
        };
        self.record_repository.add(record)
    }
}

pub struct GetRecordService<'a> {
    record_repository: &'a dyn RecordRepository,
}

impl<'a> GetRecordService<'a> {
    pub fn new(record_repository: &dyn RecordRepository) -> GetRecordService {
        return GetRecordService { record_repository };
    }

    pub fn run(&self, query: GetRecordQuery) -> Result<Record, String> {
        return self.record_repository.get(query.key);
    }
}

pub enum ListResult {
    KeyView(Vec<String>),
    RecordView(Vec<Record>),
}

pub struct ListRecordsService<'a> {
    record_repository: &'a dyn RecordRepository,
}

impl<'a> ListRecordsService<'a> {
    pub fn new(record_repository: &dyn RecordRepository) -> ListRecordsService {
        return ListRecordsService { record_repository };
    }

    pub fn run(&self, query: ListRecordsQuery) -> ListResult {
        let all_records = self.record_repository.all();
        if query.keys_only {
            let keys: Vec<String> = all_records.iter().map(|v| v.key.to_string()).collect();
            ListResult::KeyView(keys)
        } else {
            ListResult::RecordView(all_records)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::record::infrastructure::repositories::InMemoryRepository;

    use super::*;

    #[test]
    fn add_new_record_adds_record() {
        let record_repository: InMemoryRepository = InMemoryRepository::new();
        let (test_key, test_val) = ("test-key", "test-val");
        let query = AddNewRecordQuery::new(test_key.to_string(), test_val.to_string());
        let service = AddNewRecordService::new(&record_repository);
        service.run(query);

        let expected_record = Record::new(&test_key.to_string(), &test_val.to_string());
        match record_repository.get(test_key.to_string()) {
            Ok(record) => assert_eq!(record, expected_record),
            Err(_) => assert!(false),
        }
    }
}