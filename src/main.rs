use crate::try_outs::{functions};

// use crate::try_outs::functions::Feline;
pub mod try_outs;

fn main() {

    let txt = String::from("A string to be converted into pig latin 98number");
    let pig_latin = functions::from_string_to_pig_latin(&txt);
    println!("{txt}");
    println!("{pig_latin}");
}