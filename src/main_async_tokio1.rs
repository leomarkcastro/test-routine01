#![deny(clippy::all)]

use tokio::time::{sleep, Duration};

async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "Arthur".to_string()
}

#[tokio::main]
async fn main() {
    let name = call_api_one().await;
    println!("Hello {:}", name);
}
