use std::io; // lib for input
use reqwest; // lib for requests
#[tokio::main] // needed for async
async fn main() {
    let mut webhook = String::new();
    println!("Enter webhook:");
    io::stdin().read_line(&mut webhook).unwrap(); // Reads input

    let client = reqwest::Client::new(); // creates api client

    loop {
        client.post(webhook.trim())
            .json(&serde_json::json!({"content": "```Github.com/Fogmalol```"})).send().await.unwrap();
    }
}
