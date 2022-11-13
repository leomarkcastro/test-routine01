#![deny(clippy::all)]

fn main() {
    let mut name: Option<String> = Option::None;
    // let mut name: Option<String> = Option::Some("hello".to_string());

    match name.as_mut() {
        Some(n) => {
            *n = format!("[[{}]]", n);
            println!("{}", n);
        }
        None => {
            println!("Non Existent");
        }
    }

    let unwrap_name = name.unwrap_or_else(|| "hi".to_string());

    println!("{}", unwrap_name);
}
