use tokio::{join, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    println!("Start");

    let (raw, processed) = join!(
        fetch_data(),
        transform_data(raw)  // ❌ won't compile: raw is undefined
    );

    println!("Processed: {}", processed);
}

async fn fetch_data() -> String {
    sleep(Duration::from_secs(2)).await;
    "kartik_raw".to_string()
}

async fn transform_data(data: String) -> String {
    sleep(Duration::from_secs(1)).await;
    data.to_uppercase()
}

use tokio::{join, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    let raw = fetch_data();
    let processed =  transform_data("kartik_raw".to_string()); // seperate input

    let(raw_result, processed_result) = join!(raw, processed);

    println!("Raw: {}", raw_result);
    println!("Processed: {}", processed_result);
}

async fn fetch_data() -> String {
sleep(Duration::from_secs(5)).await;
"fetched_data".to_string()
}

async fn transform_data(data: String) -> String{
    sleep(Duration::from_secs(5)).await;
    data.to_uppercase()
}

use tokio::{join, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    println!("Starting both tasks...");

    let task1 = async_task("task 1", 3);
    let task2 = async_task("task 2", 2);

    let (result1, result2) = join!(task1, task2);

    println!("Results :{}, {}", result1, result2);
}

async fn async_task(name: &str, seconds: u64) -> String {
    println!("{} started, will take {}s", name, seconds);
    sleep(Duration::from_secs(seconds)).await;
    format!("{} done", name)
}

use tokio::{join, time::{sleep, Duration}};

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