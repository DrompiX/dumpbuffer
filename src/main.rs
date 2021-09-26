use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
enum DumpBufferCLI {
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Add {
        key: String,
        value: Vec<std::ffi::OsString>,
    },
    Get {
        key: String,
    },
    List {
        #[structopt(long)]
        keys_only: bool,
    },
}

impl DumpBufferCLI {
    fn joined_value(&self, separator: Option<&str>) -> Option<String> {
        let sep = separator.unwrap_or(" ");
        match self {
            DumpBufferCLI::Add { key: _, value } => {
                let vals: Vec<String> = value
                    .iter()
                    .map(|v| {
                        let result_string = v.to_owned().into_string();
                        result_string.expect(format!("Cant parse value {:?}", v).as_str())
                    })
                    .collect();
                Some(vals.join(sep))
            }
            _ => None,
        }
    }
}

fn main() {
    let args = DumpBufferCLI::from_args();
    println!("{}", args.joined_value(None).unwrap());
}
