fn main() {
    // Error Handling
    // 2 types of errors: Recoverable and Unrecoverable
    // Unrecoverable - panic! macro
    // Recoverable - Result<T, E> enum

    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}
