mod ui;

use crate::ui::cli;
use structopt::StructOpt;

fn main() {
    let args = cli::DumpBufferCLI::from_args();
    println!("{}", args.joined_value(" ").unwrap());
}
