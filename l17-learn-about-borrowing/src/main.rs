fn main() {

    // References allow us to `borrow` values without taking ownership of them.

    let greeting = String::from("hello");
    let greeting_reference = &greeting;  // We `borrow` greeting but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`

    

    // Reference in functions
    // ======================

    fn print_greeting(message: &String) {
        println!("Greeting: {}", message);
    }

    let greeting = String::from("hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again


    // Mutating borrowed values
    // =========================

    // What happened if we try to mutate a value we borrowed?

    fn change(message: &String) {
        // Compile Error: cannot borrow `*message` as mutable, as it is behind a `&` reference
        // `message` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        // message.push_str("!"); // We try to add a `!` to the end of our message
    }

    let greeting = String::from("hello");
    change(&greeting);

    // Fix
    fn change_2(message: &mut String) {
        message.push_str(", world!")
    }
    let mut greeting_mut = String::from("hello");
    change_2(&mut greeting_mut);
    println!("Mut greeting: {}", greeting_mut);

    // IMPORTANT:
    // With `&` borrows known as `immutable borrows`, we can read the data but we can't change it.
    // `&mut` borrows known as mutable borrows, we can both read and write the data.


    // Borrowing and mutable references
    // =================================

    // Immutable and mutable references differ in one other way that has radical effects on how we build our Rust programs.
    // When borrowing a value of any type `T`, the following rules apply:

    // IMPORTANT:
    // Your code must implement `either` of the following definitions, but **not both at the same time**:

    // * One or more immutable references (`&T`)
    // * Exactly one mutable reference (`&mut T`)


    // Counter-examples, the following code doesn't allow definitions, so the compilation fails:

    let mut value = String::from("hello");

    let ref1 = &mut value;
    let ref2 = &mut value;
    // Compile Error:      ^^^^^^^^^^ second mutable borrow occurs here
    // error[E0499]: cannot borrow `value` as mutable more than once at a time

    println!("{}, {}", ref1, ref2);


    // We can even try to mix immutable references with mutable references, but the compiler will still complain:

    let mut value = String::from("hello");
    let ref1 = &value;
    let ref2 = &mut value;
    // Compile Error:      ^^^^^^^^^^ second mutable borrow occurs here
    // error[E0502]: cannot borrow `value` as mutable because it is also borrowed as immutable

    println!("{}, {}", ref1, ref2);

    // This restriction may seem harsh at first, but this aspect of borrowing prevents Rust code from a whole
    // host of issues including never having a race condition.

}
