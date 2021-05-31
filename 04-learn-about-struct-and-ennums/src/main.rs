fn main() {

    // Struct
    // =================

    // Structs in Rust come in three flavours: classic structs, tuple struct and unit structs.

    // A struct with named fields
    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool,
    }

    // A tuple struct
    struct Point2D(u32, u32);

    // A unit struct
    struct Unit;
    // Unit structs are most commonly used as markers.


    // Instantiate Struct
    // ==================

    // After we define a struct, we use it by creating an instance and specifying concrete values for each field.

    let person = Person {
        name: String::from("admin"),
        age: 25,
        likes_oranges: true,
    };

    let orgin = Point2D(0, 0);

    let unit = Unit;

    if person.likes_oranges {
        println!("{:?} is {:?} and likes oranges.", person.name, person.age)
    } else {
        println!("{:?} is {:?} and doesn't like oranges.", person.name, person.age)
    }


    // Enum
    // ==========

    enum WebEvent {
        PageLoad,
        PageUnload,

        KeyPress(char),
        Paste(String),

        Click{x: i64, y: i64},
    }

}
