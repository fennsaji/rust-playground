use std::future::{Future, Ready};

// Synchronous function
fn sync_function(input: i32) -> i32 {
    input * 2
}

// Wrapping synchronous function with a future
fn async_function(input: i32) -> impl Future<Output = i32> {
    // Create a future that immediately resolves with the result of the synchronous function
    std::future::ready(sync_function(input))
}

// Example usage
#[tokio::main]
async fn main() {
    // Call the async function
    let result = async_function(5).await;
    println!("Result: {}", result);
}