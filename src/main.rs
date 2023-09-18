use crate::try_outs::{functions};

// use crate::try_outs::functions::Feline;
pub mod try_outs;

fn main() {

    // Instantiate an empty mutable string for input
    let mut str_input = String::new();
    let mut input_text = String::new();
    
    loop {
        // Invite user to input a record
        println!("Input a record of an employee");
    
    
        // Taking user input into a mutable string
        std::io::stdin().read_line(&mut str_input).expect("Input error");
    
    
        input_text = match functions::check_input_format(&str_input) {
            Ok(text) => text.clone(),
            Err(err) => {
                println!("Input error : {err}");
                continue;
            }
        }; 
        
        break;
    }

    println!("Your input : {input_text}");
}