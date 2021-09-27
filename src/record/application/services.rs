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
            key: query.key.as_str(),
            value: query.value.as_str(),
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

    pub fn run(&self, query: GetRecordQuery) -> Record {
        return self.record_repository.get(query.key);
    }
}

pub enum ListResult<'a> {
    KeyView(Vec<&'a str>),
    RecordView(Vec<Record<'a>>),
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
            let keys: Vec<&'a str> = all_records.iter().map(|v| v.key).collect();
            ListResult::KeyView(keys)
        } else {
            ListResult::RecordView(all_records)
        }
    }
}
