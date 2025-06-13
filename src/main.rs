use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");
    
    let data = fetch_data().await;

    println!("Got from fetch_data(): {}", data);
}

async fn fetch_data() -> String {
    println!("Fetching data...");
    sleep(Duration::from_secs(2)).await;
    println!("Done fetching.");

    "This is your data".to_string()
}