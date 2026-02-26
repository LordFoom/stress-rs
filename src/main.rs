use std::thread;

use crate::cli::CliArgs;
use crate::stressor::Stressor;
use crate::stressor::stress;
use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;

mod cli;
mod stressor;

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    let num_threads = args.threads;
    let url_to_stress = args.url.clone();
    let optional_description = args.description.clone();
    let mut vec_threads = Vec::new();
    for i in 1..=num_threads {
        let our_url = url_to_stress.clone();
        let our_desc = optional_description.clone();
        let jh = tokio::spawn(async move {
            let mut binding = Stressor::default();
            let stressor = binding
                .optional_description(our_desc)
                .thread_num(i)
                .url(our_url);
            stress(stressor).await
        });
        vec_threads.push(jh);
    }

    for th in vec_threads {
        th.await
            .map_err(|_| anyhow!("Thread panicked"))?
            .map_err(|e| anyhow!("Stress failed: {e}"))?
    }
    Ok(())
}
