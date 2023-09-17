use crate::try_outs::{functions};

// use crate::try_outs::functions::Feline;
pub mod try_outs;

fn main() {

    let arr = [-5, 4, 1, -3, 2, 6, 4, -9];

    println!("Original array");
    dbg!(arr);

    let median = functions::median_from_int_array(&arr[..]);

    dbg!(median);

    let mode = functions::mode_from_int_array(&arr[..]);

    dbg!(mode);

    // println!("Median : {median} | Mode : {mode}");

}