#![deny(clippy::all)]

use rand::prelude::*;

fn get_full_name() -> &'static str {
    "John Doe"
}

fn get_random_name<'a>(a: &'a str, b: &'a str) -> &'a str {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();

    // println!("{}", n1);

    if n1 > 100 {
        return a;
    }

    b
}

fn get_age() -> i8 {
    5i8
}

struct Person<'person> {
    name: &'person str,
    friend: &'person str,
}

impl<'a> Person<'a> {
    fn whos_the_friend(&self) -> &str {
        self.friend
    }
}

fn main() {
    let friend = get_random_name("Josh", "Jane");
    let full_name = get_full_name();
    let age = get_age();

    println!("{} and {}. Age {}", friend, full_name, age);

    let person = Person {
        name: full_name,
        friend,
    };

    println!(
        "Person {} has a friend named {}",
        person.name,
        person.whos_the_friend()
    );
}
