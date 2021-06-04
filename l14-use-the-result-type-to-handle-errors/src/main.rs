fn main() {
    // Rust provides `Result<T, E>` type for returning and propagating errors.
    // For convention, the `Ok<T>` variant represents a success and contains a value,
    // the `Err<T>` variant represents an error and contains a error value.

    // The `Result<T, E>` enum is defined as:

    // enum Result<T, E> {
    //     Ok(T): // A value T was obtained
    //     Err(E): // An error of type E was encountered instead.
    // }

    // In contrast to the `Option` type, which describes the possibility of the absence of a value,
    // the `Result` type is best suited whenever failure are expected.

    // The `Result` type also has `unwrap` and `expect` methods, which do either of following:
    // - Return the value inside the `Ok`, if this is the case
    // - Cause the program to panic, if the variant is an `Err`

    // Example function:
    // Return values:
    // - A `Result` value with an `Ok` variant that carries the result of a successful division.
    // - An `Err` variant that carries a struct `DivisionByZeroError` which signals an unsuccessful division.

    // NOTE: This part is a macro to tell the Rust compiler to make the type printable for debugging purpose.
    #[derive(Debug)]

    struct DivisionByZeroError;

    fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 {
            Err(DivisionByZeroError)
        } else {
            Ok(dividend / divisor)
        }
    }

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));

}
