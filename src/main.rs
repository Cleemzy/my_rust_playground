fn main() {
    // // First, we declare a type which has `iter` method to get the `Iter` struct (`&[usize]` here):
    // let slice = &[1, 2, 3];

    // // Then, we iterate over it:
    // for element in slice.iter() {
    //     println!("{element}");
    // }

    let txt = String::from("Heyy");
    for c in txt.chars(){
        println!("{}",c);
    };
}
