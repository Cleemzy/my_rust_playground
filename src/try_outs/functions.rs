use std::collections::HashMap;

// Implementation of the 3rd execise in 8.3
pub mod employee_management{
use std::{io::Error, io::ErrorKind, collections::HashMap, hash::Hash};

    pub fn input_and_manage(){
            // Instantiate an empty mutable string for input
            let mut str_input = String::new();
            let mut input_text = String::new();
            
            // Hardcoding the departments into a vector
            let departments = vec!["Engineering","Accounting","HR"];

            // Hardcoding the employees into a vector
            let mut employees: Vec<HashMap<String, String>> = hardcode_employees();

            loop {
                // Invite user to input a record
                println!("Input a record of an employee");
            
            
                // Taking user input into a mutable string
                std::io::stdin().read_line(&mut str_input).expect("Input error");
            
            
                input_text = match check_input_format(&str_input, &departments) {
                    Ok(text) => text.clone(),
                    Err(err) => {
                        println!("Input error : {err}");
                        str_input.clear();
                        continue;
                    }
                }; 
                
                // Gets out of the loop if there is no input error
                break;
            }

            add_employee(&mut employees, &str_input);

            let employees_nb = employees.iter().count();

            dbg!(employees);
            dbg!(employees_nb);

            // println!("Your input : {input_text}");
        
            // dbg!(input_text);
    }

    // Add user input into new employee having a department
    fn add_employee(employees: &mut Vec<HashMap<String, String>>, input_txt: &str){
        let (name, dep) = get_name_and_department(&(input_txt.to_string()));

        employees.push(HashMap::from([
            (String::from("name"), name),
            (String::from("department"), dep)
        ]));

    }


    // Get a tuple of the name and the department from the user input
    fn get_name_and_department(input_txt: &String) -> (String, String){
        let words: Vec<&str> = input_txt.split_whitespace().collect();

        let name = words.get(1).unwrap();
        let department = words.get(3).unwrap();

        let conventional_name: String = word_to_conventional(name); // First char of name to uppercase
        let conventional_department: String = word_to_conventional(department); // First char of department to uppercase

        (conventional_name, conventional_department)
    }

    // First char of name to uppercase
    fn word_to_conventional(text: &str) -> String{
        // This line makes the first char to uppercase
        let first_char = text.chars().next().unwrap().to_uppercase().next().unwrap();
        let other_chars = &text[1..];

        format!("{first_char}{other_chars}")
   
    }
    
    // Checking input format
    pub fn check_input_format<'a>(input_txt: &'a String, departments: &'a Vec<&str>) -> Result<&'a String, Error>{
    
        let not_none_input = check_input_if_none(&input_txt)?;
        let input_4words_ensured = ensures_string_has_4_words(&not_none_input)?;
        let input_correct_words = ensures_words_are_correct(&input_4words_ensured, departments)?;
    
        Result::Ok(input_4words_ensured)
    }
    
    pub fn ensures_words_are_correct<'a>(input_txt: &'a String, departments: &Vec<&str>) -> Result<&'a String, Error> {
        //  Split the input_text into words
        let words: Vec<&str> = input_txt.split_whitespace().collect();
    
        // Raises the below error if the first word is not "add"
        if words.get(0).unwrap().to_lowercase() != "add" {
            println!("oyy");
            return Err(Error::new(ErrorKind::InvalidInput, "The first word should be : \"Add\" or \"add\""));
        };
    
        // Raises the below error if the third word is not "to"
        if words.get(2).unwrap().to_lowercase() != "to" {
            return Err(Error::new(ErrorKind::InvalidInput, "The third word should be : \"to\""));
        };
    
        // Getting the departments list in lowercase string and pushing them into this new empty Vec
        let mut lowercase_departments_as_string: Vec<String> = Vec::new();
        for department in departments{
            lowercase_departments_as_string.push(department.to_lowercase());
        }

        // Checks inside the departments list if it contains the input department name, else returns the error below
        let department_placeholder = words.get(3).unwrap().to_lowercase();
        if ! lowercase_departments_as_string.contains(&department_placeholder){
            let error_format = format!("{department_placeholder} is not found in the departments list");
            return Err(Error::new(ErrorKind::InvalidInput, error_format));
        };
            
    
        Result::Ok(input_txt)
    }
    
    // Ensures input string has 4 words
    pub fn ensures_string_has_4_words(input_txt: &String) -> Result<&String, Error>{
        //  Words counter
        let words_nb = input_txt.split_whitespace().count();
    
        // Raise error if the input doesn't contain 4 words
        if words_nb != 4{
            return Err(Error::new(ErrorKind::InvalidInput, "Needs 4 words : the format is \"Add [employee] to [department]\" "));
        };
        Result::Ok(input_txt)
    }
    
    // Checks if input string is not none (0 word)
    pub fn check_input_if_none(input_txt: &String) -> Result<&String, Error>{
        let words_nb = input_txt.as_str().split_whitespace().count();
        if words_nb <= 0{
            return Err(Error::new(ErrorKind::InvalidInput, "Input empty"));
        };
        Result::Ok(input_txt)
    }

    pub fn hardcode_employees() -> Vec<HashMap<String, String>> {
        // ["Engineering","Accounting","HR"]

        // Instantiate an empty Vec of employee Hashmaps
        let mut employees: Vec<HashMap<String, String>> = Vec::new();

        // Creating employees 
        let employee1: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Fabrice")),
            (String::from("department"), String::from("Accounting"))
        ]);

        let employee2: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Sarah")),
            (String::from("department"), String::from("HR"))
        ]);

        let employee3: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Jim")),
            (String::from("department"), String::from("Engineering"))
        ]);

        let employee4: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Marc")),
            (String::from("department"), String::from("Accounting"))
        ]);

        let employee5: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Paulette")),
            (String::from("department"), String::from("HR"))
        ]);

        let employee6: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Gus")),
            (String::from("department"), String::from("Engineering"))
        ]);

        let employee7: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Rodrigo")),
            (String::from("department"), String::from("Accounting"))
        ]);

        let employee8: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("William")),
            (String::from("department"), String::from("HR"))
        ]);

        let employee9: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Hebert")),
            (String::from("department"), String::from("Engineering"))
        ]);
        
        employees.push(employee1);
        employees.push(employee2);
        employees.push(employee3);
        employees.push(employee4);
        employees.push(employee5);
        employees.push(employee6);
        employees.push(employee7);
        employees.push(employee8);
        employees.push(employee9);
        
        employees
    }
}

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