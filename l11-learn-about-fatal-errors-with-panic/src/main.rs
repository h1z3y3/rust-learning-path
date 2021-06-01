fn main() {

    // panic!()
    // ====================

    // You can use the `panic!()` macro to panic the current thread.
    // It prints an error message, unwinds and cleans up the stack,
    // and then exits the program.

    panic!("Farewell!");

    // This program will exit with status code 101 and print the error message.

    // `thread 'main' panicked at 'Farewell!', src/main.rs:10:5`

    // The preceding panic message reveals the place in the source code where the panic occurred.

    // In general terms, you should use `panic!` when a program reaches an unrecoverable state
    // meaning anything where there is absolutely no way to recover from the error.



    // Rust panics on some operations such as division by zero or access an index
    // that isn't present in an array, a vector or a hash map,
    // as shown in the following code:

    let v = vec![0, 1, 2, 3];
    println!("{}", v[6])

}
