enum Foo<T> {
    None,
    ThirdVariant(i64),
    Some { x: T, y: i32 },
}

enum Option<T> {
    None,
    Some(T),
}

fn bruh() {
    let foo = Foo::Some {
        x: Box::new(7),
        y: 9,
    };

    match foo {
        Foo::None => todo!(),
        Foo::Some { x, y } => println!("{x} {y}"),
        Foo::ThirdVariant(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bruh_test() {
        bruh();
        panic!();
    }
}
