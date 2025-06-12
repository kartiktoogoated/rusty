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
