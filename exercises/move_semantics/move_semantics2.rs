// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    //   Make another, separate version of the data that's in `vec0` and pass that
    //    to `fill_vec` instead.
    {
        fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
            let mut vec = vec;

            vec.push(22);
            vec.push(44);
            vec.push(66);

            vec
        }

        let vec0 = Vec::new();

        let mut vec1 = fill_vec(vec0.clone());

        println!("solution 1:");

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }
    // 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
    //    and then copy the data within the function in order to return an owned
    //    `Vec<i32>`
    {
        fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
            let mut vec = vec.clone();

            vec.push(22);
            vec.push(44);
            vec.push(66);

            vec
        }

        let vec0 = Vec::new();

        let mut vec1 = fill_vec(&vec0);

        println!("solution 2:");

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }
    // 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
    //    mutable), modify it directly, then not return anything. Then you can get rid
    //    of `vec1` entirely -- note that this will change what gets printed by the
    //    first `println!`
    {
        fn fill_vec(vec: &mut Vec<i32>) -> &Vec<i32> {
            vec.push(22);
            vec.push(44);
            vec.push(66);

            vec
        }

        let mut vec0 = Vec::new();

        fill_vec(&mut vec0);

        println!("solution 3:");

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    }

}


