// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    //for x in option {

    // sort of like a "guard"
    // would be nice to assume "if let x = option" assumed the truthy/succesful variant of Option
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
