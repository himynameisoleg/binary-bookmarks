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

// use std::ops::Deref;
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
// fn main() {
//     // let x = 5;
//     // let y = MyBox::new(x);
//     //
//     // assert_eq!(5, x);
//     // assert_eq!(5, *y); // *(y.deref())
//
//     let m = MyBox::new(String::from("Rust"));
//     hello(&m);
//     // because of Deref coersion we turn &MyBox<String> into &String into &str
//     // otherwise we would need to explicitly dereference with `hello(&(*m)[..];`
//     // to match all of the signitures
// }
//
// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

// Code Cleanup with `Drop` trait
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");
    // c.drop(); // this doesnt work: explicit destructor calls not allowed
    drop(c); // allowed
    println!("CustomSmartPointer dropped before end of main.")
}
