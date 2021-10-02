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

    pub fn run(&self, query: &AddNewRecordQuery) -> Result<(), String> {
        let record = Record {
            key: query.key.to_string(),
            value: query.value.to_string(),
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

    pub fn run(&self, query: &GetRecordQuery) -> Result<Record, String> {
        return self.record_repository.get(query.key.to_string());
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

    pub fn run(&self, query: &ListRecordsQuery) -> Result<ListResult, String> {
        self.record_repository.all().and_then(|records| {
            if query.keys_only {
                let keys: Vec<String> = records.iter().map(|v| v.key.to_string()).collect();
                Ok(ListResult::KeyView(keys))
            } else {
                Ok(ListResult::RecordView(records))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::record::infrastructure::repositories::InMemoryRecordRepository;

    use super::*;

    #[test]
    fn add_new_record_adds_record() {
        let record_repository: InMemoryRecordRepository = InMemoryRecordRepository::new();
        let query = AddNewRecordQuery::new("test_key".to_string(), "test_val".to_string());
        let service = AddNewRecordService::new(&record_repository);
        service.run(&query).unwrap();

        let expected_record = Record::new(&query.key, &query.value);
        match record_repository.get(query.key) {
            Ok(record) => assert_eq!(record, expected_record),
            Err(_) => assert!(false),
        }
    }
}
