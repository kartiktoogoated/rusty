use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");
    sleep(Duration::from_secs(5)).await;
    println!("5 seconds later");
    delay().await;
}

async fn delay() {
    sleep(Duration::from_secs(10)).await;
    println!("Hey after 10 seconds");
}

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");

    // .await suspends here without blocking the thread
    sleep(Duration::from_secs(2)).await;

    println!("2 Seconds Later");
}

// with tokio
use tokio::{join, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    println!("Starting async task...");

    simple_async().await;

    println!("Async task finished");
}

async fn simple_async() {
    println!("Inside simple_async(), sleeping for 3 seconds");
    sleep(Duration::from_secs(3)).await;
    println!("Done sleeping");
}