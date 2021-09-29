mod record;
mod shared;

use structopt::StructOpt;

use crate::record::application::{
    queries::{AddNewRecordQuery, GetRecordQuery, ListRecordsQuery},
    services::{AddNewRecordService, GetRecordService, ListRecordsService, ListResult},
};
use crate::record::infrastructure::repositories::InMemoryRepository;
use crate::shared::infrastructure::cli::DumpBufferCLI;

fn handle(args: &DumpBufferCLI) -> String {
    let record_repository: InMemoryRepository = InMemoryRepository::new();
    match args {
        DumpBufferCLI::Add { key, value: _ } => {
            let joined_value = args.joined_value(" ").unwrap();
            let query = AddNewRecordQuery::new(key.to_string(), joined_value);
            let service = AddNewRecordService::new(&record_repository);
            service.run(query);
            format!("Successfully added new value with key {}", key)
        }
        DumpBufferCLI::Get { key } => {
            let query = GetRecordQuery::new(key.to_string());
            let service = GetRecordService::new(&record_repository);
            match service.run(query) {
                Ok(record) => record.value.to_string(),
                Err(e) => e,
            }
        }
        DumpBufferCLI::List { keys_only } => {
            let query = ListRecordsQuery::new(keys_only.clone());
            let service = ListRecordsService::new(&record_repository);
            match service.run(query) {
                ListResult::KeyView(keys) => {
                    format!("[\n  {}\n]", keys.join(",\n  "))
                }
                ListResult::RecordView(records) => {
                    let repr: Vec<String> = records.iter().map(|v| v.to_string()).collect();
                    format!("{}", repr.join(",\n"))
                }
            }
        }
    }
}

fn main() {
    let args = DumpBufferCLI::from_args();
    let result = handle(&args);
    println!("{}", result)
}
