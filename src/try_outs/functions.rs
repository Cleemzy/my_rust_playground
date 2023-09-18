use std::collections::HashMap;

// Converts string to pig latin 
pub fn from_string_to_pig_latin(text: &String) -> String{
    // Instantiate an empty string to take the text to be converted
    let mut pig_latin = String::new();

    // Split the text removing all whitespace
    let words = text.as_str().split_whitespace();

    // Iterating over the words
    for word in words{
        // Get the first char
        let first_ch = word.chars().next().unwrap();

        // The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
        if is_char_consonant(first_ch){
            let other_chars = String::from(&word[1..]);
            let first_ch_as_string = String::from(first_ch).as_str().to_lowercase();
            pig_latin.push_str(format!("{other_chars}-{first_ch_as_string}ay ").as_str());
            continue; 
        }

        // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
        if is_char_vowel(first_ch){
            let other_chars = String::from(&word[..]);
            pig_latin.push_str(format!("{other_chars}-hay ").as_str());
            continue;
        }

        // In case it's neither starting with a consonant nor a vowel (a number for example)
        // Just returns the word as it is
        pig_latin.push_str(format!("{word} ").as_str());

    }

    // Trim off unnecessary whitespace
    String::from(pig_latin.as_str().trim())
}

// Checks if a char is a consonant
fn is_char_consonant(ch: char) -> bool{
    let consonants: [char; 42] = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
        'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'
    ];

    consonants.contains(&ch)

}

// Checks if a char is a vowel
fn is_char_vowel(ch: char) -> bool{
    let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    vowels.contains(&ch)
}


// Return the mode of an array of integers
pub fn mode_from_int_array(nbs: &[i32]) -> i32{
    // convert the integers array into an integers vector 
    let vect = nbs.to_vec();

    // dbg!(vect);

    // Instantiate the map in which we will see the occurence of each value 
    let mut hm:HashMap<i32, i32> = HashMap::new();

    // Inserting the values in Vec inside the map and each key has its occurence as value
    for nb in &vect{
        // See if the key is already inserted inside the map
        // If not insert it having a default value of 0
        // Either way, it returns the pointer of the value of ths key and we increment it by dereferencing it
        *hm.entry(*nb).or_insert(0) +=1;
    }


    // Instantiate the max value in which we will compare to each of the value of the hashmap hm
    let mut max = single_value_from_hash_map(&hm);

    // Finding the max inside the hashmap
    for (_, v) in &hm{
        // find for each iteration the max
        if *v > max {max = *v}
    };

    // Return the max corresponding key from the hashmap, hence, the mode
    for (k, v) in &hm {
        if *v == max {return *k;}
    }

    -1
}

// Get one value from a single iteration of hashmap if <i32, i32>
fn single_value_from_hash_map(hm: &HashMap<i32, i32>) -> i32 {
    for (_, v) in hm{
        return *v;
    };
    -1
}


// Return the median of an array of integers 
pub fn median_from_int_array(nbs: &[i32]) -> i32 {
    // convert the integers array into an integers vector 
    let mut vect = nbs.to_vec();

    
    // Sort the vector
    vect.sort();
    
    println!("And this is the sorted vector from the array");
    dbg!(vect.clone());


    // The median is the value at the middle of the vector when sorted
    vect[vect.len() / 2]
}

#[derive(Debug)]
pub enum Feline{
    Cat,
    Tiger
}

enum Animals{
    Canine,
    Feline(Feline)
}

#[derive(Debug)]
enum FlagshipBrand{
    Apple{definition: String, phone_model: String},
    Samsung(String)
}

fn area(rec: Rectangle) -> u32 {
    rec.width * rec.height
}

impl Rectangle{
    // area method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn reverse_rectangle(rec: &Rectangle) -> Self{
        Self { width: rec.height, height: rec.width }
    }

    fn reverse(&mut self){
        let h = self.height;
        let w = self.width;

        self.height = w;
        self.width = h;
    }
}

// Rectangle struct

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

struct User{
    username: String,
    email: String,
    age: u32 
}

fn print_user(u: &User){
    println!("username: {}", u.username);
    println!("email: {}", u.email);
    println!("age: {}", u.age);
}

fn change_str(s: &mut String){
    s.push_str(" p")
}

// return indexes of values in vector which sum is target 
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Instance of the vec result to be pushed in the loop 
    let mut result: Vec<i32> = Vec::new();

    // Enumerate through the vector using index and value
    for (i, v) in nums.iter().enumerate(){
        let substracted = target - v; 
        let mut outer_vec = nums.clone();
        outer_vec.remove(i);
        if outer_vec.iter().any(|&x| x == substracted){
            if let Some(j) = nums.iter().position(|&x| x == substracted){
                if j != i {
                    result.push(i as i32);
                    result.push(j as i32);
                    return result;
                }
            }
        }
    }

    return result;
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