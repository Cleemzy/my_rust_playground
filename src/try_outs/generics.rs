// Main testing function for trying traits out

use super::independents;

pub fn traits_function(){
    let toutou : Dog = Dog{name: String::from("Toutou"), breed: String::from("sheperd")};
    let mimi : Cat = Cat{name: String::from("Mimi"), breed: String::from("orange")};
    let adolf : Dolphin = Dolphin { name: String::from("Adolf"), breed: String::from("Blyatler") };

    let e_t : independents::Alien = independents::Alien{name: String::from("E.T"), race: String::from("Martian")};

    toutou.make_noise();
    mimi.make_noise();
    adolf.make_noise();
    e_t.make_noise();
    
}

// Testing traits on a common crate family (here simple example : Animal)
pub trait Animal{
    fn make_noise(&self){
        println!("..blyat BLYAT");
    }
}

// New type Dog
#[derive(Debug)]
pub struct Dog{
    name: String,
    breed: String
}

// New type Cat 
#[derive(Debug)]
pub struct Cat{
    name: String,
    breed: String
}

pub struct Dolphin{
    name: String,
    breed: String
}

impl Animal for Dolphin{}

impl Animal for Dog {
    fn make_noise(&self) {
        println!("Wouf wouf");
    }
}

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Miaouw miaouw");
    }
}

// 

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