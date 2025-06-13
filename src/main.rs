use tokio::{time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    println!("Fetching data...");
    let raw_data = fetch_data().await;

    println!("Transforming data...");
    let final_output = transform_data(raw_data).await;

    println!("Final output: {}", final_output);
}

async fn fetch_data() -> String {
    println!("Waiting 3s");
    sleep(Duration::from_secs(3)).await;
    "hello from fetch".to_string()
}

async fn transform_data(data: String) -> String {
    println!("Processing");
    sleep(Duration::from_secs(2)).await;
    format!("{} -> transformed", data)
}