fn main() {
    println!("Hello world");
    another_function();

    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(17, 0), false);
    assert_eq!(is_divisible_by(24, 6), true);
}

fn another_function() {
    println!("hello from another function!");
}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }

    // In Rust, the last expression inside a code block `{...}` is always returned,
    // so we don't need to use the `return`

    // But don't use the semicolon
    dividend % divisor == 0
}

