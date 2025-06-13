use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// A custom feature that prints and returns after being polled once
struct MyFuture;

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling MyFuture...");
        Poll::Ready("Its done")
    }
}

fn main() {
    let mut fut = MyFuture;

    let waker = futures::task::noop_waker(); // fake waker to make it compile
    let mut cx = Context::from_waker(&waker);

    // Pin the future and poll it manually
    let mut pinned = unsafe { Pin::new_unchecked(&mut fut) };
    let result = pinned.as_mut().poll(&mut cx);

    println!("Got: {:?}", result);
}

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