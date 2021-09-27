mod record;

use crate::record::infrastructure::cli;
use structopt::StructOpt;

fn main() {
    let args = cli::DumpBufferCLI::from_args();
    println!("{}", args.joined_value(" ").unwrap());
}
