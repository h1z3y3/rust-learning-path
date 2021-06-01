fn main() {

    // Option<T>
    // ========================


    // The Rust standard library provides an `Option<T>` enum to be used
    // when the absence of a value is a possibility.
    // `Option<T>` is widely used in Rust code because it encodes very common scenario
    // in which a value could be something or it could be nothing.


    // Option<T> manifests itself as one of two variant:

    // enum Option<T> {
    //     None, // The value doesn't exist
    //     Some(T), // The value exist
    // }

    // `None` and `Some` are not types but rather variants of the `Option<T>` type.
    // The `<T>` part of `Option<T>` enum declaration states that the `T` is generic
    // and will be associated with the `Some` variant of the `Option` enum.

    // Accessing a vector's non-existent index would cause the program to `panic`,
    // but you could avoid that by using `Vec::get` method,
    // which returns an `Option` type instead of panicking.
    // If the value exists at the specified index, it's wrapped in `Option::Some(T)` variant.
    // If the index is out of bounds, it would return a `Option::None` value instead.

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent); // returned a `None` value instead of panicking


    // Pattern matching
    // ========================

    // Rust has an extremely powerful control flow operator called `match`,
    // which you can use to compare a value against a series of patterns and then
    // execute the code based on which pattern matches.

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    for &index in [2, 4, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :(")
        }
    }

    // You could stress the fact that coconuts are awesome by running the following:

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            // Using &"coconut" because `fruits.get(index)` returns an `Option<&&str>`
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :(")
        }
    }

    // Whenever you use the `match` expression, keep the following rules in mind:
    //  - `match` arms are evaluated from top to bottom. Specific case must be defined
    //     earlier than generic cases or they'll never be matched and evaluated.
    //  - `match` arms must cover every possible value that the input type could have.
    //     You'll get a compiler error if you try to match against non-exhaustive pattern list.


    // The `if let` expression
    // ==========================

    // Rust offers a convenient way to test whether a value conforms with a single pattern.


    // Consider the following example, which matches on an `Option<u8>` value but
    // want to execute code only if the value is 7.

    let some_number: Option<u8> = Some(7);
    match some_number  {
        Some(7) => println!("That's my lucky number."),

        // Using `_` (underscore) wildcard pattern after all other patterns to match `anything else`
        // and it's used to satisfy the compiler demands for exhausting match arms.
        _ => {}
    }

    // More easier way, the following code behaves as the precious one:
    if let Some(7) = some_number {
        println!("That's my lucky number");
    }

    // The nice thing about `if let` expression is that you don't need
    // all the boilerplate code of a `match` expression when you're interested in a single
    // pattern to match aganst.


    // Use `unwrap` and `expect`
    // =========================

    // `unwrap`

    // To access the inner value of an `Option` type directly by using `unwrap` method.
    // * Be careful, though, because this method will panic if the variant is a `None`

    // For instance:

    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    // `expect`
    // The `expect` method does the same as `unwrap`, but it provide a custom panic message
    // that's provided by its second argument:

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    b.expect("fruits are healthy"); // Panic with `fruits are healthy`


    // Because `unwrap` and `expect` method might panic, we don't recommend using them.
    // Instead, consider either of the following approaches:

    // - Use pattern matching and handle the `None` case explicitly.
    // - Call similar non-panicking methods, such as `unwrap_or`, which returns a default value
    //   if the variant is `None` or the inner value if the variant is `Some(value)`

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
