// Implementation of the 3rd execise in 8.3
pub mod employee_management{
use std::{io::Error, io::ErrorKind, collections::HashMap};

    pub fn employee_management(){
            // Instantiate an empty mutable string for input
            let mut str_input = String::new();
            
            // Hardcoding the departments into a vector
            let departments = vec!["Engineering","Accounting","Sales"];

            // Hardcoding the employees into a vector
            let mut employees: Vec<HashMap<String, String>> = hardcode_employees();

            // Loop until a the input of the new employee is correct 
            loop {
                // Invite user to input a record
                println!("Input a record of an employee");
            
            
                // Taking user input into a mutable string
                std::io::stdin().read_line(&mut str_input).expect("Input error");
            
            
                match check_new_employee_input_format(&str_input, &departments) {
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

            // Push the employees Vec to add the new input one
            add_employee(&mut employees, &str_input);

            // Taking user input into a mutable string
            let mut dep_input = String::new();

            // Loop until a the input of the department to get the employees is correct
            loop {
                // Asking user to select a department to get all entries in that department
                println!("Now would you list all entries in a selected department ? Select one among exiting departments : ");
                for dep in departments.clone(){
                    print!("[{dep}] ");
                };
    
                
                std::io::stdin().read_line(&mut dep_input).expect("Input error");
                
                match check_list_in_departments_input_format(&dep_input, &departments) {
                    Ok(text) => text.clone(),
                    Err(err) => {
                        println!("Input error : {err}");
                        dep_input.clear();
                        continue;
                    }
                }; 

                // Gets out of the loop if there is no input error
                break;
            }

            let mut filtered_employees = filter_employees_by_department(&dep_input, &employees);

            
            filtered_employees.sort_by(|a, b| a.get("name").unwrap().cmp(b.get("name").unwrap()));
            employees.sort_by(|a, b| { 
                let department_a = a.get("department").unwrap();
                let department_b = b.get("department").unwrap();
                let name_a = a.get("name").unwrap();
                let name_b = b.get("name").unwrap();
                
                // Compare by department first
                let department_comparison = department_a.cmp(department_b);
                
                // If the departments are the same, compare by name
                if department_comparison == std::cmp::Ordering::Equal {
                    name_a.cmp(name_b)
                } else {
                    department_comparison
                }
            });
            
            println!("List of employees in {dep_input} sorted alphabetically :");
            for employee in  &filtered_employees  {
                let name = employee.get("name").unwrap();
                let department = employee.get("department").unwrap();
                let text = format!("Name : {name} - Department : {department}");
                println!("{text}");
            }

            println!("\r");

            println!("List of all employees grouped by department and sorted alphabetically:");
            for employee in  &employees  {
                let name = employee.get("name").unwrap();
                let department = employee.get("department").unwrap();
                let text = format!("Name : {name} - Department : {department}");
                println!("{text}");
            }

            // dbg!(employees);
            // dbg!(filtered_employees);
            
            
    }

    // Get all employees by the department input
    fn filter_employees_by_department(dep_input: &String, employees: &Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>>{
        // Department key string
        let department_key = String::from("department");
        
        // Gets the correct department string value
        let conventional_dep_input: String = word_to_conventional(&dep_input);
 
        // Instantiate the returned vector of employees
        let mut filtered_list: Vec<HashMap<String, String>> = Vec::new();
        
        // Loop over the employees list and push users having the corresponding department name into the new list
        for employee in employees.clone(){
            
            if employee.get(&department_key).unwrap() == &conventional_dep_input.trim(){
                filtered_list.push(employee);
            }
        }

        filtered_list
    }


    // Checks the input department if it is correct 
    pub fn check_list_in_departments_input_format<'a>(input_txt: &'a String, departments: &'a Vec<&str>) -> Result<&'a String, Error>{
    
        let checked_input = check_if_dep_input_is_correct(input_txt)?;

        // Getting the departments list in lowercase string and pushing them into this new empty Vec
        let mut lowercase_departments_as_string: Vec<String> = Vec::new();
        for department in departments{
            lowercase_departments_as_string.push(department.to_lowercase());
        }

        // Checks inside the departments list if it contains the input department name, else returns the error below
        let department_placeholder: String = checked_input.trim().to_lowercase();
        if ! lowercase_departments_as_string.contains(&department_placeholder){
            let error_format = format!("{department_placeholder} is not found in the departments list");
            return Err(Error::new(ErrorKind::InvalidInput, error_format));
        };
    
        Result::Ok(checked_input)
    }

    // Checks if input of department is correct (1 word only)
    pub fn check_if_dep_input_is_correct(input_txt: &String) -> Result<&String, Error>{
         //  Words counter
         let words_nb = input_txt.split_whitespace().count();
    
         // Raise error if the input doesn't contain 4 words
         if words_nb != 1{
             return Err(Error::new(ErrorKind::InvalidInput, "Type any of these existing departments"));
         };
        Result::Ok(input_txt)
    }


    // Add user input into new employee having a department
    fn add_employee(employees: &mut Vec<HashMap<String, String>>, input_txt: &str){
        let (name, dep) = get_name_and_department(&(input_txt.to_string()));

        println!("You successfully added {name} to {dep} !");

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
    pub fn check_new_employee_input_format<'a>(input_txt: &'a String, departments: &'a Vec<&str>) -> Result<&'a String, Error>{
    
        let not_none_input = check_input_if_none(&input_txt)?;
        let input_4words_ensured = ensures_string_has_4_words(&not_none_input)?;
        let input_correct_words = ensures_words_are_correct(&input_4words_ensured, departments)?;
    
        Result::Ok(input_correct_words)
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
        // ["Engineering","Accounting","Sales"]

        // Instantiate an empty Vec of employee Hashmaps
        let mut employees: Vec<HashMap<String, String>> = Vec::new();

        // Creating employees 
        let employee1: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Fabrice")),
            (String::from("department"), String::from("Accounting"))
        ]);

        let employee2: HashMap<String, String> = HashMap::from([
            (String::from("name"), String::from("Sarah")),
            (String::from("department"), String::from("Sales"))
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
            (String::from("department"), String::from("Sales"))
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
            (String::from("department"), String::from("Sales"))
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

pub mod simple_functions{
    pub fn do_things(){
        // println!("Do things");
        let text1 :&str = "heho";
        let text2: &str = "hehoo";

        let longest = longest_str(text1, text2);

        println!("The longest is : {}", longest);
    }

    pub fn longest_str<'a>(text1: &'a str, text2: &'a str) -> &'a str{
        if text1.len() < text2.len(){text2}else{text1}
    }
}