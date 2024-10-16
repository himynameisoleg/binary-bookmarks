use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Error Handling
    // 2 types of errors: Recoverable and Unrecoverable
    // Unrecoverable - panic! macro
    // Recoverable - Result<T, E> enum

    // panic!("crash and burn");
    //run with  RUST_BACKTRACE=1 cargo run to see full stack trace
    // let v = vec![1, 2, 3];
    // v[99];

    // Recoverable Errors with Result
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // Handling Different Kinds of Errors
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e);
    //             }
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // Alternative to the above
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
