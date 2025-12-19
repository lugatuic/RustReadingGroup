// // Some Examples of undefined behavior:
// // 1. Use-after-free
// // 2. Index out-of-bounds
// // 3. Reading uninitialized memory
// //  - `int x; printf("%d", x);`
// //  - `int *p = malloc(sizeof(int)); printf("%d", *p);`
// // 4. dereferencing null
// // 5. overflowing a signed integer in c

// use std::mem::MaybeUninit;

// fn foo() {
//     let x: i32 = unsafe { MaybeUninit::uninit().assume_init() };
//     println!("{x}");
//     println!("Hello");
// }

// fn bar() {
//     // This example never quite worked. But what I was going for was here: https://marabos.nl/atomics/basics.html#undefined-behavior
//     let index = 3;
//     let a = [123, 456, 789];
//     let b = unsafe { *a.get_unchecked(index) };
//     println!("{b}");
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     pub fn test_foo() {
//         foo();
//         // panic!();
//     }

//     #[test]
//     pub fn test_bar() {
//         // bar();
//     }
// }
