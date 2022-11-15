#![deny(clippy::all)]

use futures::executor::block_on;

async fn get_name() -> String {
    "John".to_string()
}

fn main() {
    let name = block_on(get_name());
    println!("Hello {}", name);
}
