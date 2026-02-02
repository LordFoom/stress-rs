use std::thread;

use crate::cli::CliArgs;
use crate::stressor::Stressor;
use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;

mod cli;
mod stressor;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let num_threads = args.threads;
    // let url_to_stress = args.url;
    let optional_description = args.description;
    let mut vec_threads = Vec::new();
    for i in 1..=num_threads {
        let jh = thread::spawn(|| {
            let stressor = Stressor::default()
                .optional_description(optional_description)
                .thread_num(i)
                .url(args.url);
        });
        vec_threads.push(jh);
    }

    for th in vec_threads {
        th.join().map_err(|_| anyhow!("Thread panicked"))?;
    }
    Ok(())
}
