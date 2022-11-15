#![deny(clippy::all)]

use std::cell::Cell;

struct Person {
    name: String,
    age: Cell<u8>,
}

impl Person {
    fn increment_age(&self) -> u8 {
        self.age.set(self.age.get() + 1);
        self.age.get()
    }
}

fn main() {
    let p1 = Person {
        name: "John".to_string(),
        age: Cell::new(20),
    };

    let new_age = p1.increment_age();
    println!("{}", new_age);
}
