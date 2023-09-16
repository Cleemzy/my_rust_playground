use std::collections::HashMap;

// use crate::try_outs::functions::Feline;
pub mod try_outs;


fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Green");
    let field_value = 40;

    scores.insert(field_name, field_value);

    dbg!(scores);

}