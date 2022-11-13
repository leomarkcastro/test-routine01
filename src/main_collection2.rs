#![deny(clippy::all)]

fn main() {
    let values = vec![1, 2, 3, 4, 5];

    let multipled_by_2: Vec<i32> = values.iter().map(|v| v * 2).collect();

    let multipled_by_2_2: Vec<i32> = multipled_by_2.iter().map(|v| v * 2).collect();

    println!("{:?}", multipled_by_2_2);

    for n in multipled_by_2_2 {
        println!("{}", n);
    }
}
