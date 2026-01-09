// https://doc.rust-lang.org/book/ch13-01-closures.html for more info.
fn foo(b: Box<i32>) {
    let f = |x: i32| {
        drop(b);
        return x;
    };
    println!("{}", f(6));
    // This causes an error:
    //println!("{}", f(6));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        foo(Box::new(5));
        panic!("Always fails so we see output.");
    }
}
