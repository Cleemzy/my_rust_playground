use crate::try_outs::{functions};

// use crate::try_outs::functions::Feline;
pub mod try_outs;

fn main() {

    let arr = [-5, 4, 1, -3, 2, 6, 4, -9];

    println!("This is the original array");
    dbg!(arr);

    let median = functions::median_from_int_array(&arr[..]);

    println!("And this is the median :");
    dbg!(median);

    functions::mode_from_int_array(&arr[..]);

}