use crate::cli::CliArgs;
use clap::Parser;

mod cli;
mod stressor;

fn main() {
    let args = CliArgs::parse();
    args.url;
}
