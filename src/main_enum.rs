#![deny(clippy::all)]

enum AnimalType {
    Cat { noise: String, mewl: bool },
    Dog { noise: String, bark: bool },
    // Rabbit,
}

struct Pet {
    name: String,
    age: u8,
    animal_type: AnimalType,
}

impl Pet {
    fn describe(&self) {
        println!("{} is {} years old", self.name, self.age)
    }

    fn make_noise(&self) {
        match &self.animal_type {
            AnimalType::Dog { noise, bark } => {
                println!("{}. Can bark? {}", noise, bark)
            }
            AnimalType::Cat { noise, mewl } => {
                println!("{}. Can mewl? {}", noise, mewl)
            }
        }
    }
}

fn main() {
    let pet = Pet {
        name: "Johnny".to_string(),
        age: 3,
        animal_type: AnimalType::Cat {
            noise: "Meow".to_string(),
            mewl: true,
        },
    };

    let pet2 = Pet {
        name: "Pol".to_string(),
        age: 3,
        animal_type: AnimalType::Dog {
            noise: "Arf!".to_string(),
            bark: true,
        },
    };

    pet.describe();
    pet.make_noise();
    pet2.describe();
    pet2.make_noise();
}
