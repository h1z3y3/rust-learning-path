use std::collections::HashMap;

fn main() {
    // Besides tuples, Rust has many other compound types that can group multiple values into one single type.

    // Array
    // ==============

    // In other words, arrays have a fixed length.
    // Every element of an array must be of the same type.

    // An array can be defined in two ways:
    // - A comma-separated list inside brackets
    // - The initial value, followed by a semicolon, and then the length of the array in brackets

    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    println!("First weekday is {}", weekdays[0]);


    let byte_buffer = [0_u8; 512];
    println!("Buffer element 100 is {}", byte_buffer[99]);

    // Array is useful when you want your data allocated on stack rather than the heap.
    // They're also useful when you want to ensure you always have a fixed number of elements.


    // Vector
    // ====================

    // store data in the heap

    // Just like array, you can use vector with the type `Vec<T>` to store multiple values of the same type.
    // Unlike arrays, vectors can grow or shrink at any time.
    // This capability is implied in their size not being known at compile time, so Rust can't prevent you from
    // accessing an invalid position in your vector.


    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}!", three_numbers);

    let ten_zeros = vec![0; 10];
    println!("Then zeros: {:?}!", ten_zeros);

    // Vectors can also be created by using the `Vector::new()` method.
    // You can push values onto the end of a vector, which will grow the vector as needed.

    // push
    let mut new_v = Vec::new();
    new_v.push(5);
    new_v.push(6);

    println!("Current vector is {:?}", new_v);

    // pop
    let pop = new_v.pop();
    println!("Current vector is {:?}, the popped value is {:?}", new_v, pop);

    // Hash maps
    // ====================

    // store data in the heap
    // Like vectors, hash maps are growable, store data in the heap, and access its items are check at run time

    let mut book_reviews: HashMap<String, String> = HashMap::new();
    book_reviews.insert(
        "key1".to_string(),
        String::from("value1"),
    );

    book_reviews.insert(
        "key2".to_string(),
        "value2".to_string(),
    );

    println!("The hash map is: {:?}", book_reviews);

    let v = book_reviews.get("key1");
    println!("The value from hash is: {:?}", v)

}
