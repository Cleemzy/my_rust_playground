fn main() {

    let text = String::from("textd to work with");

    let index = first_whitespace_index_or_last_index(&text);

    println!("{index}");

}

// Return the index of the first whitespace in a string, or the last index in case of no whitespace
fn first_whitespace_index_or_last_index(s: &String) -> usize{
    let s_slice = s.as_bytes(); // get the bytes slice from the borrowed string 's' in parameter

    // loop over the slice as enumeration 
    for (i, &c) in s_slice.iter().enumerate() {
        if c == b' ' {
            return i; // return the index 'i' when we meet a whitespace
        }
    }
    return s_slice.len(); // return the slice's last index when no whitespace found in loop above
    
}