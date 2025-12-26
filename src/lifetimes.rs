pub fn returns_param<'a>(x: &'a i32) -> &'a i32 {
    return &x;
}

#[cfg(test)]
mod test {
    use crate::lifetimes::returns_param;

    #[test]
    fn uses_returns_param() {
        let x = 10;
        println!("{}", returns_param(&x));
        // let y_ref;
        // {
        //     let y = 7;
        //     y_ref = returns_param(&y);
        // }
        // println!("{y_ref}");
    }
}
