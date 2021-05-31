fn main() {

    // if / else expression
    // =========================

    if 1 == 2 {
        println!("whoops, mathematics broke")
    } else {
        println!("everything is find!")
    }

    // Unlike in most language, `if` blacks can also act as expressions.
    // Remember that all branches must return the same type for our code to compile.

    let formal = true;
    let greeting = if formal {
        "Good morning."
    } else {
        "Hello, friend!"
    };

    println!("{}", greeting); // prints "Good morning."

    // else if expression:

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
    // Output:
    // number is divisible by 3


    // Loop forever with loop
    // ============================

    // A `loop` expression denotes an infinite loop.
    // It repeats execution of ite body continuously.

    // loop {
    //     println!("I loop forever")
    // }

    // Unlike the other kinds of loops in Rust like while and for,
    // `loop` can be used in expressions that return value via `break`

    let mut i = 1;
    let something = loop {
        i = i * 2;
        if i > 100 {
            break i;
        }
    };

    assert_eq!(something, 128);


    // While loop
    // ====================

    // The following code loops until the predicate evaluates to `true`

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }


    // Iterate with for loop
    // ==========================

    // Some values can be iterated over directly
    // and others can produce iterators by calling methods like `iter()`.

    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("The value is: {}", elem);
    }

    // Another easy way to create an iterator is to use the range notation `a..b`
    // This notation yields values from `a`(inclusive) to `b`(exclusive) in step of one.

    for item in 0..5 {
        println!("{}", item * 2);
    }
}
