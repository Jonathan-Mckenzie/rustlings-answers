// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)


// reordered to be before main, not sure if there are any other tricks
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
