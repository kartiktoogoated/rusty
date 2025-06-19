use std::future::Future;

// A function that takes a string slice with a lifetime and returns a future
async fn print_with_lifetime<'a>(msg: &'a str) {
    println!("Message: {}", msg);
}

// A function that returns a future, showing how lifetimes can be used with async
fn async_with_lifetime<'a>(msg: &'a str) -> impl Future<Output = ()> + 'a {
    async move {
        print_with_lifetime(msg).await;
    }
}

#[tokio::main]
async fn main() {
    let message = String::from("Hello, lifetimes with async!");
    async_with_lifetime(&message).await;
}
