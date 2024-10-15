use std::fs::File;

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
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}
