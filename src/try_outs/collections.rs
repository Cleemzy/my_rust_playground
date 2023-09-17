pub mod vectors{
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
}


pub mod strings{
    
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = String::from(" Chooms!");

    // CONCATENATION ALWAYS OWNS THE FIRST STRING AND THE OTHERS SHOULD BE OF TYPE &str
    // let s4 = s1 + &s2 + &s3;

    // STRING CONCATENATION USING THE 'format!' macro
    // let s1 = String::from("Hey");
    // let s2 = String::from("Buddy"); 
    // let f = format!("{s1} | {s2}"); // Do not take ownership of any of its parameters (They're all references)

    // ITERATING OVER A STRING AS chars
    // let hello: &str = "Здравствуйте";

    // for c in hello.chars(){
    //     println!("{c}");
    // }

    // ITERATING OVER A STRING AS bytes
    // let hello: &str = "Здравствуйте";

    // for c in hello.bytes(){
    //     println!("{c}");
    // }


}

pub mod hash_maps{



    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // USING HASHMAP TO COUNTS HOW OFTEN A WORD APPEARS IN THE STRING SLICE

    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let field_name = String::from("Green");
    // let field_value = 40;

    // scores.insert(field_name, field_value);
}