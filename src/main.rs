use crate::first::List;

pub mod first;

fn main() {
    let x = Box::new(54);
    baz(x);
    // We discussed this error in detail! It's related to ownership ([covered in "the book", here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)).
    println!("{x}");
}

fn baz(x: Box<i32>) {
    println!("{x}");
}
