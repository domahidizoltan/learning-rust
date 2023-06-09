use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

pub fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let i:i32 = num.into();
    println!("Number into i32 is {:?}", i)
}