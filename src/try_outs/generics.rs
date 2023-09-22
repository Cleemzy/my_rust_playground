pub fn generic_function(){
    
    println!("Generic function");

    let mut u1 = User{
        name: String::from("Sam"),
        notes : 70
    };

    let mut u2 = User{
        name: String::from("Momo"),
        notes : 5
    };

    let mut u3 = User{
        name: String::from("Hal"),
        notes : 70.98
    };

    // dbg!(u1);
    let utuple = u1.exchange_note(&mut u3);

    dbg!(u1);
    dbg!(u3);
    dbg!(utuple);




}
// Definition of User struct to work with
#[derive(Debug)]
pub struct User<T>{
    name: String,
    notes: T
}


impl<T: Copy> User<T>{

    // Method implemented to exchange notes from one another
    pub fn exchange_note<Y: Copy>(&self, other: &User<Y>) -> (User<Y>,User<T>){
        
        (User {name: self.name.clone(), notes: other.notes},
        User {name: other.name.clone(), notes: self.notes})

    }
    
}