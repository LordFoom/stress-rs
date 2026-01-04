use clap::Parser;

#[derive(Parser, Debug)]
// #[command(0.0.1, "Simple rust stress tester")]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    ///The url of the server we want to stress
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    threads: usize,
}
