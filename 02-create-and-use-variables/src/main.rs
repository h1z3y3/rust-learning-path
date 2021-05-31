fn main() {
    // =========================
    // Create a variable

    let a_number = 10;
    let a_boolean = false;

    // add exclamation to let Rust know we're using this function
    // as a macro and not a normal function
    println!("the number is {}!", a_number);
    println!("the  boolean is {}!", a_boolean);


    // =========================
    // Mutability

    let a_number_not_mutable = 10;
    println!("The number is {}.", a_number_not_mutable);

    // Error: cannot assign twice to immutable variable
    // a_number_not_mutable = 15;
    // println!("Now the number is {}.", a_number_not_mutable);


    // =========================
    // mutable variables

    let mut a_number_mutable = 10;
    println!("The mutable number is {}.", a_number_mutable);

    a_number_mutable = 15;
    println!("Not the mutable variable is {}!", a_number_mutable);


    // ==========================
    // Shadowing

    // You can a declare a new variable with the same name of the previous variable, which creates
    // a new binding. In Rust, this operation is called "shadowing" because the new variable
    // shadowing the previous variable. The old variable is still exists, but you can't refer it
    // in this scope anymore.

    // The variable doesn't need to be mutable. No mutation occurs because every operation creates
    // a new variable while shadowing the previous one.


    let number = 10;

    let number = number + 1;

    let number = number * 2;

    println!("Finally the number is {}!", number)
    

    



}
