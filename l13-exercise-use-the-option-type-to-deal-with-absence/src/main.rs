// In this exercise, you'll finish implementing a function
// that receives a `Person` struct and return a `String` that contains its full name

// Keep in mind that some people don't have a middle name but, if they do, if must be included in the return value.

// You must edit only the `build_full_name` function. Note that the part that
//handles the first and last names has already been implemented for you.

struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    // TODO: Implement the part of this function that handles the person's middle name.

    // My work here:
    // 1.
    // if let Some(middle_name) = &person.middle {
    //     full_name.push_str(middle_name);
    //     full_name.push_str(" ");
    // }

    // 2.
    match &person.middle {
        Some(middle_name) => {
            full_name.push_str(middle_name);
            full_name.push_str(" ");
        }
        _ => (),
    }

    full_name.push_str(&person.last);
    full_name
}

fn main() {
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}


