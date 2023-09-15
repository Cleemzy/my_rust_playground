
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