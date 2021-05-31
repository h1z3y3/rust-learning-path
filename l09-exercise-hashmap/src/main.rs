fn main() {

    // In this exercise, you need to defined a basket of fruits in form of a hash map.

    // The key represents the name of the fruit.
    // The value represents how many of that particular fruits in the basked.

    // You have to put at least three different types of fruits in the basket.
    // For example, three types might be apples, bananas, and mango.
    // The total count of the fruits should be at least five.

    // You only need to edit the `fruit_basket` function in this exercise.


    let basket = fruit_basket();

    assert!(
        basket.len() > 3,
        "basket must have at least 3 types of fruits."
    );

    assert!(
        // Note: must declare sum return value type, <u32>
        basket.values().sum::<u32>() > 5,
        "basket must have at least 5 fruits."
    );

}

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // let mut basket = todo!("declare your hash map here.");
    let mut basket: HashMap<String, u32> = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.

    // My work:
    basket.insert(String::from("apple"), 10);
    basket.insert(String::from("mango"), 20);

    basket
}
