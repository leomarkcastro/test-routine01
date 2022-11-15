#![deny(clippy::all)]

trait CanRun {
    fn run(&self);
}

impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

struct Person {
    name: String,
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}

fn main() {
    let people = vec![
        Person {
            name: "Jose".to_string(),
        },
        Person {
            name: "Jane".to_string(),
        },
    ];

    people.run();
}
