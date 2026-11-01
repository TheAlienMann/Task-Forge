mod cli;

use clap::Parser;
use cli::{create_formatter, Cli};
use std::process;

fn main() {
    let cli = Cli::parse();
    let format = cli.output_format();

    if let Err(err) = cli::run(cli) {
        let formatter = create_formatter(format);
        formatter.format_error(&err);
        process::exit(1);
    }
}
