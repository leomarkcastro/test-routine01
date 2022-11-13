#![deny(clippy::all)]

fn main() {
    let age: Option<i32> = Option::Some(20);

    let age_mul_2 = age.map(|v| v * 2);

    println!("{}", age_mul_2.unwrap_or_default());
}
