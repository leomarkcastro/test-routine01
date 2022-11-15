#![deny(clippy::all)]

use std::ops::AddAssign;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn move_offset(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let mut p1 = Point { x: 5.0, y: 5.6 };
    p1.move_offset(2.4, 2.9);
    println!("{:?}", p1);
    let p2 = Point { x: 0.45, y: 5.6 };
    p1 += p2;
    println!("{:?}", p1);
}
