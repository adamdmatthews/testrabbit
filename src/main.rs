#![warn(clippy::all, clippy::pedantic)]

use std::time::Duration;

use clap::{Parser, value_parser};
use tokio::time::interval;

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = 5, value_parser = value_parser!(u64).range(1..))]
    seconds: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let name = whoami::realname().unwrap_or_else(|_| whoami::username().unwrap_or("World".into()));
    let mut ticker = interval(Duration::from_secs(args.seconds));
    loop {
        ticker.tick().await;
        println!("Hello, {name}!");
    }
}
