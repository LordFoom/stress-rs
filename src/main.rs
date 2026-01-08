use std::thread;

use crate::cli::CliArgs;
use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;

mod cli;
mod stressor;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let num_threads = args.threads;
    let url_to_stress = args.url;
    let mut vec_threads = Vec::new();
    for i in 1..=num_threads {
        let jh = thread::spawn(|| {});
        vec_threads.push(jh);
    }

    for th in vec_threads {
        th.join().map_err(|_| anyhow!("Thread panicked"))?;
    }
    Ok(())
}
