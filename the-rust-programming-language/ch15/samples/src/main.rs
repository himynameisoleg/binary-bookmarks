use crate::List::{Cons, Nil};

// Cons (construct functions) used in functional languages as recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // this isnt really useful
    // let b = Box::new(5);
    // println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
