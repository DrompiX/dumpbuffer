#![allow(dead_code)]

pub struct AddNewRecordQuery {
    pub key: String,
    pub value: String,
}

impl AddNewRecordQuery {
    pub fn new(key: String, value: String) -> AddNewRecordQuery {
        return AddNewRecordQuery { key, value }
    }
}

pub struct GetRecordQuery {
    pub key: String,
}

impl GetRecordQuery {
    pub fn new(key: String) -> GetRecordQuery {
        return  GetRecordQuery { key };
    }
}

pub struct ListRecordsQuery {
    pub keys_only: bool,
}

impl ListRecordsQuery {
    pub fn new(keys_only: bool) -> ListRecordsQuery {
        return ListRecordsQuery { keys_only };
    }
}

#[derive(Debug)]
pub struct DeleteRecordQuery {
    pub key: Option<String>,
    pub all: bool,
}

impl DeleteRecordQuery {
    pub fn new(key: &Option<String>, all: bool) -> DeleteRecordQuery {
        return DeleteRecordQuery { key: key.to_owned(), all };
    }
}