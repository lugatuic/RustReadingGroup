pub fn bar(x: i32) {
    println!("{:?}", x);
}
pub fn foo() {
    let x = 1;
    bar(x);
    bar(x);
}
