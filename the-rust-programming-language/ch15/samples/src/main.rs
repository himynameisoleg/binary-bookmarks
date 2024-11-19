// use crate::List::{Cons, Nil};
//
// // Cons (construct functions) used in functional languages as recursive types
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// fn main() {
//     // this isnt really useful
//     // let b = Box::new(5);
//     // println!("b = {}", b);
//
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())
}
