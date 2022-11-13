#![deny(clippy::all)]

fn main() {
    let message: [&str; 2] = ["foo", "bar"];

    let appended = message.iter().map(|m| format!("[{}]", m));

    for m in message.iter() {
        println!("{}", m);
    }
    for m in appended {
        println!("{}", m);
    }

    let mut vector_int: Vec<i8> = vec![1, 2, 3];
    let mut vector2_int: Vec<i8> = vec![4, 5, 6];

    vector_int.push(4);

    println!("{:?}", vector_int);

    vector_int.clear();

    println!("V1: {:?}", vector_int);
    println!("V2: {:?}", vector2_int);

    println!("V1 contains 4 {}?", vector_int.contains(&4));
    println!("V2 contains 4 {}?", vector2_int.contains(&4));

    vector_int.append(&mut vector2_int);

    println!("V1: {:?}", vector_int);
    println!("V2: {:?}", vector2_int);

    println!("V1 contains 4 {}?", vector_int.contains(&4));
    println!("V2 contains 4 {}?", vector2_int.contains(&4));
}
