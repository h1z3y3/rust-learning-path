fn main() {

    // Remember that tuples and arrays have different indexing notation.

    indexing_tuple();

    indexing_array();
}

// Use `tuple.x` to index a tuple
fn indexing_tuple() {

    let numbers = (1, 2, 3);

    // TODO - Correct the syntax
    // let second = todo!("Replace with the tuple indexing syntax")

    // My fix:
    let second = numbers.1;

    assert_eq!(
        2, second,
        "This is not the second number is the tuple: {}",
        second
    )
}

// Use `array[x]` to index a array
fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];

    // TODO - Correct the syntax
    // let letter_d = todo!("Replace with the array indexing syntax")

    // My fix
    let letter_d = characters[3];

    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d,
    )
}

