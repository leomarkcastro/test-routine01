#![deny(clippy::all)]

struct Person {
    name: String,
    age: u32,
}

fn create_person(name: &str, age: u32) -> Person {
    Person {
        name: name.to_string(),
        age,
    }
}

struct Point(f64, f64, f64);

impl Point {
    fn describe(&self) {
        println!("x: {} || y: {} || z: {}", self.0, self.1, self.2)
    }
}

fn main() {
    let person = create_person("Leo Mark", 22);
    let person2 = Person {
        name: "Lea Ann".to_string(),
        ..person
    };

    println!("{} is {} years old", person.name, person.age);
    println!("{} is {} years old", person2.name, person2.age);

    let points = Point(5.0, 10.0, 15.0);

    points.describe();
}
