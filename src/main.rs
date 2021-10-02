mod record;
mod shared;

use dirs;
use structopt::StructOpt;

use crate::record::infrastructure::repositories::KVFileDatabaseRepository;
use crate::shared::infrastructure::file_db::KVFileDatabase;
use crate::record::application::{
    queries::{AddNewRecordQuery, GetRecordQuery, ListRecordsQuery},
    services::{AddNewRecordService, GetRecordService, ListRecordsService, ListResult},
};
use crate::shared::infrastructure::cli::DumpBufferCLI;

fn handle(args: &DumpBufferCLI) -> String {
    let mut db_path = dirs::home_dir().expect("Could not determine user home directory");
    db_path.push(".dumpb_store");
    let db = KVFileDatabase::new(&db_path);
    let record_repository = KVFileDatabaseRepository::new(&db);

    match args {
        DumpBufferCLI::Add { key, value: _ } => {
            let joined_value = args.joined_value(" ").unwrap();
            let query = AddNewRecordQuery::new(key.to_string(), joined_value);
            let service = AddNewRecordService::new(&record_repository);
            match service.run(&query) {
                Ok(_) => format!("Successfully added new value with key {}", key),
                Err(e) => format!("[ERROR] {}", e),
            }
            
        }
        DumpBufferCLI::Get { key } => {
            let query = GetRecordQuery::new(key.to_string());
            let service = GetRecordService::new(&record_repository);
            match service.run(&query) {
                Ok(record) => record.value.to_string(),
                Err(e) => format!("[ERROR] {}", e),
            }
        }
        DumpBufferCLI::List { keys_only } => {
            let query = ListRecordsQuery::new(keys_only.clone());
            let service = ListRecordsService::new(&record_repository);
            match service.run(&query) {
                Ok(ListResult::KeyView(keys)) => {
                    format!("[\n  {}\n]", keys.join(",\n  "))
                }
                Ok(ListResult::RecordView(records)) => {
                    let repr: Vec<String> = records.iter().map(|v| v.to_string()).collect();
                    format!("{{\n{}\n}}", repr.join(",\n"))
                }
                Err(e) => e.to_string()
            }
        }
    }
}

fn main() {
    let args = DumpBufferCLI::from_args();
    println!("{}", handle(&args))
}
