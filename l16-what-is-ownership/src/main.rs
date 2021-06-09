fn main() {

    // In Rust, memory is managed through an ownership system, which is set of rules checked at compile time.
    // None of the ownership features slow down your program while it's running.

    // Scoping Rules
    // ========================

    // NOTE:
    // In Rust, `variable` are often called `binding`. This is because `variables` in Rust aren't very variable.
    // they don't change that often since they're immutable by default. Instead, we often think about names being
    // `bound` to data, hence the name `binding`. We'll use both `variable` and `binding` interchangeable though.


    // `mascot` is not valid and can not be used here, because it's not yet declared.
    {
        let mascot = String::from("ferris"); // `mascot` is valid from this point forward.
        // do stuff with mascot
    }
    // this scope is now over, so `mascot` is no longer valid and cannot be used.


    // Ownership and dropping
    // =========================

    // Whenever an object goes out of the scope, it's `dropped`
    // Dropping a variable means releasing any resources that are tied to it.

    // In our example above, `mascot` variable owns the String data associated with it.
    // The `String` itself owns the heap-allocated memory that holds the characters of the string.

    // At the end of the scope, `mascot` is dropped, the `String` it owns is dropped,
    // and finally, the memory that `String` owns is freed.

    {
        let mascot = String::from("ferris");
        // `mascot` dropped here. The String data memory will be freed here.
    }


    // Move Semantics
    // ======================

    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris
        let ferris = mascot;
        // ferris dropped here. The String data memory will be freed here.
    }

    // NOTE:
    // A key thing to understand is that once ownership is transferred, the old variable is no longer valid.
    // In our example above, after we transfer the ownership of the `String` from `mascot` to `ferris`,
    // we can no longer use the `mascot` variable any more.

    // In Rust, `transfer ownership` is known as `moving`.
    // In other words, in above example, the `String` value has been moved from `mascot` to `ferris`.

    // If we try to use `mascot` after the `String` has been moved from `mascot` to `ferris`,
    // the compiler will not compile our code:

    {
        let mascot = String::from("ferris");
        let ferris = mascot;

        println!("{}", mascot); // We'll try to use the `mascot` after we've moved ownership of the `String` data from `mascot` to `ferris`.
    }


    // IMPORTANT:
    // In Rust, only one thing can ever own one piece of data at a time.


    // Ownership in function
    // ==============================

    // Let's take a look at an example of a string being passed to a function as an argument.
    // Passing something as argument to function moves that thing into the function.

    fn process(input: String) {}

    fn caller() {
        let s = String::from("Hello world!");
        process(s); // Ownership of the string in `s` moved into `proceess`
        process(s); // Error! ownership already moved.
    }

    // This pattern has profound impact on the way Rust code is written.
    // It's central to the promise of memory safety that Rust proposes.

    // In Rust, ownership transfer data (it's moving) is the default behavior.

}
