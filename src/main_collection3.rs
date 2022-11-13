#![deny(clippy::all)]

fn main() {
    let names = vec![
        "John".to_string(),
        "Arthur".to_string(),
        "Dutch".to_string(),
    ];

    let greeted_names: Vec<String> = names
        .iter()
        .filter(|n| n.len() > 4)
        .map(|n| format!("Hello {}!", n))
        .collect();

    for n in greeted_names.iter() {
        println!("{}", n);
    }
}
