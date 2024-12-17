fn main() {
    // complex if-let, else if, else match
    //
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();
    //
    // if let Some(color) = favorite_color {
    //     println!("Useing your favorite color, {}, as background", color);
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }

    // while-let example
    // let mut stack = Vec::new();
    //
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // for loop match
    // let v = vec!['a', 'b', 'c'];
    //
    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is an index {}", value, index);
    // }

    // let point = (3, 5);
    // print_coordinates(&point);

    //Refutable pattern example
    // let Some(x) = some_option_value; // this failes to comlpile
    // if let Some(x) = some_option_value {
    //     // do something... this coveres the None optoin
    // }

    // Irrefutable example
    // if let x = 5 {
    //     //this will fail to compile since this is an Irrefutable match pattern
    // }

    // Matching Literals
    // let x = 1;
    //
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     _ => println!("any"),
    // }

    // matching named vars
    // let x = Some(5);
    // let y = 10;
    //
    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Matched y = {:?}", y),
    //     _ => println!("default case"),
    // }
    //
    // println!("At the end: x={:?}, y={:?}", x, y);

    // multiple patterns
    // let x = 1;
    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("default"),
    // }

    // matching ranges
    let x = 5;
    match x {
        1..=5 => println!("one though five"),
        _ => println!("default"),
    }
}

// function param matching with tuple
// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("current location: ({}, {})", x, y);
// }
