fn main() {

    // Rust is a statically language.
    // The compiler must know exactly data type for each variable in your code.

    // The compiler usually infer the type for the value.
    // You don't need tell the type in your codes.
    // When many types are possible, you must inform the compiler the specific type by using type annotations.

    // Suppose we write a program to convert a string to a number with `.parse()` method:

    // let number: u32 = "42".parse().expect("Not a number");

    // We tell the compile we want a number to be 32-bit number by annotating the type u32 after the variable name.

    // You can experiment by removing the type annotation and observe the resulting compiler error:

    // Error, consider giving `number` a type
    // let number = "42".parse().expect("Not a number");

    // The compiler will occur some error


    // Rust built-in data types
    // ================================

    // Rust comes with some built-in types to express numbers, text and truthiness.
    // We'll cover each of these types in fhe following subtopics.

    // Integers in Rust are identified by bit size and the signed property.
    // A signed integer can be a positive or negative number.
    // An unsigned integer can be only a positive number.

    // The integer types depend on the kind of your computer you program is running on.
    // The 64-bit types is used on a 64-bit computer. 32-bit
    // If you don't specify the type for an integer, and the system can't infer the type,
    // it assigns the i32 (signed 32 bit integer) by default.

    // Rust's floating-point types are f32 and f64
    // The default type is f64.
    // On modern CPUs, the f32 type is roughly same speed as f32, but it has more precious.

    // let x = 2.0; // f64, default type
    // let y: f32 = 3.0; // f32, via type annotation

    // All Rust's primitive number types support mathematical operations such as addition, subtraction, multiplication, and division.

    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // Boolean
    // ============================

    let is_bigger = 1 > 4;
    println!("{}", is_bigger);


    // Character and strings
    // ==========================

    // The char type is most primitive type among them and is specified with single quotation marks:

    let c = 'z';
    let z = 'Z';

    // Note
    // Some language treat their char type as 8-bit unsigned integer, which is the equivalent of the Rust u8 type.
    // The char type in Rust contains unicode code points, but they don't use utf-8 encoding.
    // A char in Rust is a 21-bit integer that's padding to be 32-bit wide.
    // The char contains the plain code point value directly.


    // For now you can think of &str as a pointer to an immutable string data.
    // String literals are all of type &str


    // Rust has a second string type, String. This type is allocated on the heap.
    // When you use this type, the length of the string doesn't need to be known before the code is compile.

    // Rust ownership and borrowing system.
    // Until then, you can think of String data as string data that can change as your program runs,
    // while &str are immutable views that don't change as your program runs.

    let mut hello = String::from("Hello, ");

    hello.push('w');

    hello.push_str("orld!");

    println!("{}", hello);


    // Tuples
    // ===============================

    // A tuple is a group of values of different types collected in one compounds.
    // They have fixed length, meaning that after they're declared, they can't grow or
    // shrink in size. The type of the tuple is defined by sequence of each member's type.

    // A tuple of lenght 3:
    ("hello", 5i32, 'c');

    // The tuple has the type signature `(&'static str, i32, char')`

    // Tuples elements can be accessed by position, which is known as tuple indexing.
    // It looks like this:

    let tuple = ("hello", 5i32, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    println!("{}", tuple.0);

}
