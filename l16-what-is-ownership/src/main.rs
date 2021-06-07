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
    
    

}
