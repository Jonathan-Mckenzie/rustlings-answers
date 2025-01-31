// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let word = optional_word.unwrap() {
        println!("The word is: {}", word); // or calling the debug format ":?" will also compile for an Option
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    if let integer = optional_integers_vec.pop().unwrap().unwrap() { // nested Option from pop() and Vec of options
        println!("current value: {}", integer); // or calling the debug format ":?" will also compile for an Option
    }
}
