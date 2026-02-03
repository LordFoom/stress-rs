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
    let url_to_stress = args.url.clone();
    let optional_description = args.description.clone();
    let mut vec_threads = Vec::new();
    for i in 1..=num_threads {
        let our_url = url_to_stress.clone();
        let our_desc = optional_description.clone();
        let jh = thread::spawn(move || {
            let stressor = Stressor::default()
                .optional_description(our_desc)
                .thread_num(i)
                .url(our_url);
        });
        vec_threads.push(jh);
    }

    for th in vec_threads {
        th.join().map_err(|_| anyhow!("Thread panicked"))?;
    }
    Ok(())
}
