mod record;
mod shared;

use subprocess::Exec;

use dirs;
use structopt::StructOpt;

use crate::record::application::{
    queries::{AddNewRecordQuery, DeleteRecordQuery, GetRecordQuery, ListRecordsQuery},
    services::{
        AddNewRecordService, ClearRecordsService, GetRecordService, ListRecordsService, ListResult,
    },
};
use crate::record::domain::repositories::RecordRepository;
use crate::record::infrastructure::repositories::KVFileDatabaseRepository;
use crate::shared::infrastructure::cli::DumpBufferCLI;
use crate::shared::infrastructure::file_db::KVFileDatabase;

fn setup_repository() -> Result<Box<dyn RecordRepository>, String> {
    let mut db_path = dirs::home_dir().expect("Could not determine user home directory");
    db_path.push(".dumpb_store");
    let db = KVFileDatabase::new(&db_path)?;
    Ok(Box::new(KVFileDatabaseRepository::new(db)))
}

fn execute_command(command: String) -> Result<(), String> {
    let cmd_parts: Vec<&str> = command.split(' ').collect();
    Exec::cmd(cmd_parts[0])
        .args(&cmd_parts[1..])
        .join()
        .expect("Failed to execute command");
    Ok(())
} 

fn handle(args: &DumpBufferCLI, repo: Box<dyn RecordRepository>) -> Result<String, String> {
    match args {
        DumpBufferCLI::Add { key, value: _ } => {
            let joined_value = args.joined_value(" ").unwrap();
            let query = AddNewRecordQuery::new(key.to_string(), joined_value);
            let service = AddNewRecordService::new(&repo);
            service
                .run(&query)
                .and_then(|_| Ok(format!("Successfully added new value with key \"{}\"", key)))
        }
        DumpBufferCLI::Get { key } => {
            let query = GetRecordQuery::new(key.to_string());
            let service = GetRecordService::new(&repo);
            service
                .run(&query)
                .and_then(|record| Ok(record.value.to_string()))
        }
        DumpBufferCLI::List { keys_only } => {
            let query = ListRecordsQuery::new(keys_only.clone());
            let service = ListRecordsService::new(&repo);
            match service.run(&query) {
                Ok(ListResult::KeyView(keys)) => Ok(format!("[\n  {}\n]", keys.join(",\n  "))),
                Ok(ListResult::RecordView(records)) => {
                    let repr: Vec<String> = records.iter().map(|v| v.to_string()).collect();
                    Ok(format!("[\n{}\n]", repr.join(",\n")))
                }
                Err(e) => Err(e),
            }
        }
        DumpBufferCLI::Delete { key, all } => {
            let query = DeleteRecordQuery::new(key, all.clone());
            let service = ClearRecordsService::new(&repo);
            service.run(&query)
        }
        DumpBufferCLI::Exec { key } => {
            handle(&DumpBufferCLI::Get{ key: key.to_string() }, repo)
                .and_then(|val| execute_command(val))
                .and_then(|_| Ok("Command completed".to_string()))
        }
    }
}

fn main() {
    let args = DumpBufferCLI::from_args();
    match setup_repository().and_then(|repo| handle(&args, repo)) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("[ERROR]: {}", e),
    }
}
