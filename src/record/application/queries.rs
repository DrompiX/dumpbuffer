#![allow(dead_code)]

pub struct AddNewRecordQuery {
    pub key: String,
    pub value: String,
}

impl AddNewRecordQuery {
    fn new(key: String, value: String) -> AddNewRecordQuery {
        return AddNewRecordQuery { key, value }
    }
}

pub struct GetRecordQuery {
    pub key: String,
}

impl GetRecordQuery {
    fn new(key: String) -> GetRecordQuery {
        return  GetRecordQuery { key };
    }
}

pub struct ListRecordsQuery {
    pub keys_only: bool,
}

impl ListRecordsQuery {
    fn new(keys_only: bool) -> ListRecordsQuery {
        return ListRecordsQuery { keys_only };
    }
}
