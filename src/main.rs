#![deny(clippy::all)]

use std::ops::Deref;

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let age = BoxedValue::new(22);
    let dereffed_age = age.deref();

    println!("{}", dereffed_age);
}
