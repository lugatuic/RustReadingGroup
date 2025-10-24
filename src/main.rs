use crate::first::List;

pub mod first;

fn main() {
    let x = Box::new(54);
    baz(x);
    println!("{x}");
}

fn baz(x: Box<i32>) {
    println!("{x}");
}
