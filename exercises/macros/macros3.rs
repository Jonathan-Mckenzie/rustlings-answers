// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)


mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}


// "crate::" for IDE warning, but not necessary
// if `mod macros` was in another file, I think macros::my_macro! is necessary
fn main() {
    crate::my_macro!();
}
