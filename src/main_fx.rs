#![deny(clippy::all)]

fn greet(name: &String) {
    if name.is_empty() {
        return println!("[[ Empty string ]]");
    }
    println!("Hello, {}!", name)
}

fn clear_string(string: &mut String) {
    string.clear()
}

fn generate_name() -> String {
    "Leo".to_string()
}

fn main() {
    let mut s1 = generate_name();
    let mutable_s1 = &mut s1;
    // let immutable_s1 = &s1;

    clear_string(mutable_s1);
    greet(mutable_s1);

    let say_hello_to = |name: &str| format!("Hello {}!", name);

    println!("{}", say_hello_to("World"));
}
