#![deny(clippy::all)]

use futures::Future;

fn call_api_two() -> impl Future<Output = String> {
    let name = "James".to_string();
    async move { format!("Hello {}", name) }
}

#[tokio::main]
async fn main() {
    println!("{}", call_api_two().await);
}
