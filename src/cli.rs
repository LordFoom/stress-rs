use clap::Parser;

#[derive(Parser, Debug)]
// #[command(0.0.1, "Simple rust stress tester")]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    ///The url of the server we want to stress
    #[arg(short, long)]
    pub url: String,
    ///How many simultanious stressors?
    #[arg(short, long)]
    pub threads: usize,
    #[arg(short, long)]
    ///What description to give them
    ///TODO: store stressing's in a sqlite db
    pub description: Option<String>,
}
