#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<&str, &str> = HashMap::new();

    hm.insert("Name", "Leo Mark DC Castro");
    hm.insert("Age", "23");

    match hm.get("Name") {
        Some(value) => println!("{}", value),
        None => println!("Not found"),
    }

    for (&k, &v) in &hm {
        println!("{}: {}", k, v);
    }

    let entry = hm.entry("Age");

    match entry {
        std::collections::hash_map::Entry::Occupied(value) => {
            println!("{}", value.get())
        }
        _ => println!("Not Found"),
    }

    hm.entry("Partner").or_insert("Felix Manloloko");

    match hm.get("Partner") {
        Some(value) => println!("{}", value),
        None => println!("Not found"),
    }
}
