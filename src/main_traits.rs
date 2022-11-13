#![deny(clippy::all)]

use std::fmt;

trait HasAge {
    fn age(&self) -> u16;
}

trait HasFullName {
    fn full_name(&self) -> String;
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

trait CanDescribeSelf
where
    Self: HasFullName + HasAge,
{
    fn describe_self(&self) -> String {
        format!("{} [{} yrs old]", self.full_name(), self.age())
    }
}

fn print_details<T>(value: &T)
where
    T: HasFullName + HasAge,
{
    println!("[{}] {} years old", value.full_name(), value.age());
}

struct Person {
    first_name: String,
    last_name: String,
    age: u16,
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl HasAge for Person {
    fn age(&self) -> u16 {
        self.age
    }
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts.first().unwrap_or(&"(?)").to_string(),
            last_name: parts.get(1).unwrap_or(&"(?)").to_string(),
            age: 0,
        }
    }
}

impl CanDescribeSelf for Person {}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe_self())
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Person")
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("age", &self.age)
            .finish()
    }
}

fn main() {
    let person1 = Person {
        first_name: "Leo".to_string(),
        last_name: "Castro".to_string(),
        age: 22,
    };

    println!("---------");
    println!("{}", person1);
    println!("{:?}", person1);

    let person2 = Person::new("John");

    println!("---------");
    println!("{}", person2);
    println!("{:?}", person2);

    println!("---------");
    print_details(&person1);
    print_details(&person2);
}
