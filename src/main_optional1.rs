#![deny(clippy::all)]

fn main() {
    let mut name: Option<String> = Option::Some("hello".to_string());

    match name.as_mut() {
        Some(n) => {
            *n = format!("[[{}]]", n);
            println!("{}", n);
        }
        None => {
            println!("Non Existent");
        }
    }

    let unwrap_name = name.expect("Name is not provided");

    println!("{}", unwrap_name);
}
