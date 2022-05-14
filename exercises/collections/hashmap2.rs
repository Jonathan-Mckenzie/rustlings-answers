// hashmap2.rs

// A basket of fruits in the form of a hash map is given. The key
// represents the name of the fruit and the value represents how many
// of that particular fruit is in the basket. You have to put *MORE
// THAN 11* fruits in the basket. Three types of fruits - Apple (4),
// Mango (2) and Lychee (5) are already given in the basket. You are
// not allowed to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute the command `rustlings hint hashmap2` if you need
// hints.


use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    // given a size requirement much greater than 11, fill the basket with missing fruit
    const SIZE_REQUIREMENT: u32 = 20; //11;
    const FRUIT_KINDS: [Fruit; 5] = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    let mut needed_fruit_quantity: u32 = SIZE_REQUIREMENT - basket.values().sum::<u32>();
    if needed_fruit_quantity <= 0 {
        return;
    }

    let number_of_fruit_provided: u32 = basket.keys().len() as u32;
    if number_of_fruit_provided >= (FRUIT_KINDS.len() as u32) {
        return;
    }
    let desired_quantity_to_add_per_fruit = needed_fruit_quantity / number_of_fruit_provided;

    // must total size requirement
    // must add fruit that are missing
    for fruit in FRUIT_KINDS {
        // TODO: Put new fruits if not already present. Note that you
        // are not allowed to put any type of fruit that's already
        // present!

        // check if key is present?
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, std::cmp::min(desired_quantity_to_add_per_fruit, needed_fruit_quantity) as u32);
            needed_fruit_quantity -= desired_quantity_to_add_per_fruit;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
}
