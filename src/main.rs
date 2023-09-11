fn main() {

    let text = String::from("text to work with");

    // let first_word = first_word(&text);

    // println!("TEXT: {text}|");
    // println!("FIRST WORD OF TEXT: {first_word}|");

    println!("{text}");
    
    let ans = replace_char_by(&text, ' ', '/');

    println!("{ans}");
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

// Return the first word of a string
fn first_word(s: &String) -> String{
    let i = first_whitespace_index_or_last_index(s); // get the index to get the first word on
    String::from(&s[..i]) // return the string from the slice at the end of the index
}

// Replace a char (c) in a string (s) by another one (b) and return it in a String
fn replace_char_by(s: &String, c: char, b: char) -> String{
    let s_str = s.as_str(); // takes the string as a string slice so we can work with it
    String::from(s_str.replace(c.to_string().as_str(), b.to_string().as_str())) // returning the replaced text as string
}
