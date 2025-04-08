use reqwest::Client;
use tokio;
use reqwest::Response; // Import the Response trait
use reqwest::json; // Import the json method

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Vector of ids

    // Create a Reqwest client
    let client = Client::new();

    // Create a vector to store futures
    let futures = ids.into_iter().map(|id| {
        let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
        async move {
            // Send request for each id
            let res = client.get(&url).send().await?;
            // Parse the response body as JSON
            let body = res.json::<serde_json::Value>().await?;
            Ok(body)
        }
    });

    // Collect all the futures into a single future
    let responses = futures::future::join_all(futures).await;

    // Handle the responses here
    match responses {
        Ok(responses) => {
            println!("{:?}", responses); // Print the array of responses
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
