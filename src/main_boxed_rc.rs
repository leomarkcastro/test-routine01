#![deny(clippy::all)]

use std::rc::Rc;

fn main() {
    let array = vec!["John".to_string(), "Jane".to_string()];
    let rc = Rc::new(array);
    let weak_rc = Rc::downgrade(&rc);
    let weak_rc2 = rc.clone();
    drop(rc);

    match weak_rc.upgrade() {
        Some(rc) => println!("{:?}", rc),
        None => println!("None"),
    }
    println!("{:?}", weak_rc2);
}
