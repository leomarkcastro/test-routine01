#![deny(clippy::all)]

use futures::Future;
use tokio::time::{sleep, Duration};

fn call_api_one() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Arthur".to_string()
    }
}

fn call_api_two() -> impl Future<Output = String> {
    let name = "James".to_string();
    async move {
        format("Hello {}", name);
    }
}

#[tokio::main]
async fn main() {
    let name = call_api_one().await;
    println!("Hello {:}", name);

    call_api_two().await;
}
