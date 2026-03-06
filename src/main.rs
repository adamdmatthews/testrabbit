#![warn(clippy::all, clippy::pedantic)]

use std::time::Duration;

use tokio::time::interval;

#[tokio::main]
async fn main() {
    let mut ticker = interval(Duration::from_secs(5));
    loop {
        ticker.tick().await;
        println!("Hello, world!");
    }
}
