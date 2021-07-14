
use std::fmt::DebugList;

fn main() {

    // The use of reference presents a problem.
    // The item a reference is referring to doesn't keep track of all of its references.
    // This can lead to an issue: when the item is dropped and its resource are freed, how
    // can we be sure that there are no references that point to this now freed, and invalid, memory?

    // Language like C and C++ often have `dangling pointer` problem.
    // Rust eliminate this issue.

    // Rust's answer to this question is lifetimes.
    // They allow Rust to ensure memory safety without the performance costs of garbage collection.


    // * `'a` is the lifetime annotation for our value `x`
    // * `'b` is the lifetime annotation for our value `y`

    let x;                    // ----------+-= 'a
    {                         //           |
        let y = 42;           // -+-- 'b   |
        let x = &y;           //  |        |
    }                         // -+        |
    println!("{}", x);        //

    // Here we can see that the inner `'b` lifetime block is shorter than the outer `'b` block.

    // The Rust compiler can verify if the borrows are valid by using the `borrow checker`.
    // The borrow checker compares the two lifetime at compile time.
    // In this scenario, `x` has a lifetime of `a` but it refers to a value with a lifetime of `'b`.
    // The reference subject(`y` at lifetime `'b`) a shorter time than the reference(`x` at lifetime `'a`)
    // so the program doesn't to compile.


    // Annotating lifetimes in functions
    // =================================

    // As with types, lifetime duration are inferred by the Rust compiler.

    // There may be multiple lifetimes. When that occurs, annotate the lifetime to help the compiler
    // understand which lifetime it will use to ensure the references are valid at runtime.

    // For example, consider a function that takes two strings as its input parameters and returns the longest of them.

    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word_2(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    // fn longest_word(x: &String, y: &String) -> &String {
    //     // Compile Error:                      ^ expected named lifetime parameter
    //     return if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     };
    // }

    // It's possible that lifetime could be different whenever the function is called.
    // We don't know the concrete lifetime of the references that will be passed to our `longest_word` function,
    // and we can't determine if the reference that will be returned will always be a valid one.

    // The borrow checkers can't determine if the reference will be a valid one either.
    // It doesn't know how the input parameters' lifetime relate to the return value's lifetime.
    // This is why we need to annotate the lifetimes manually.

    // We can add generic lifetime parameters to our function signature.
    // These parameters define the relationship between the references so the borrow checkers can complete its analysis:

    fn longest_word_2<'a>(x: &'a String, y: &'a String) -> &'a String {
        return if x.len() > y.len() {
            x
        } else {
            y
        };
    }

    // Make sure to declare generic lifetime parameters inside angle brackets,
    // and add the declaration between the parameter list and the function name.


    // Let's experiment with this sample code and change some values and lifetimes of
    // the references passed in to the `longest_word_3` function to see how it behaves.
    // The compiler would also reject the following snippet, but can you guess why?

    let magic1 = String::from("abracadabra!");
    let result;
    {
        let magic2 = String::from("shazam!");
        result = longest_word_3(&magic1, &magic2);
    }
    println!("The longest magic word is {}", result);

    fn longest_word_3<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // If we inspected the code, as humans, we would see that `magic1` is longer than `magic2`.
    // We would see that the result contains a reference to `magic1`, which will live long enough to be valid.
    // However, Rust can't run that code at compile time.
    // It will consider both the `&magic1` and `&magic2` references to be possible return values and will emit
    // the error that we saw.
    
    // Annotating lifetimes in types
    // =============================

    // Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition
    // with the lifetime of each reference that it carries along with it.

    // For example, consider the following example code.
    // We have a `text` string (which owns its contents) and a `Highlight` tuple struct.
    // The struct has one field, `part`, that holds a string slice.
    // The slice is a borrowed value from another part of our program.

    #[derive(Debug)]
    struct Highlight<'document>(&'document str);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    print!("{:?}", fox);
    print!("{:?}", dog);


    // We place the name of generic lifetime parameter inside angle brackets after the name of struct.
    // This placement so we can use the lifetime parameter in the body of the struct definition.
    // IMPORTANT:
    // This instance of `Highlight` can not live longer than the reference of `part` field of the definition.

}




