mod brief_example_of_clone;
pub mod first;
mod lifetimes;
pub mod second;
mod undefined_behavior;

fn main() {
    // let x = Box::new(1);
    // baz(x);
    // // We discussed this error in detail! It's related to ownership ([covered in "the book", here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)).
    // println!("{x}");

    // drop(x);
}

// javascript
// let l = [1, 2, 3]
// let l2 = l.map((x) => x + 1)
// fn baz(x: Box<i32>) {
// if *x > 10 {
//     return;
// }
// baz(Box::new(*x + 4));
// }
