// Variables hold primitive data or references to data 
// Variables are immutable by default
// Rust is a block-scoped language 

pub fn run() {
    let name = "Rishabh";
    let mut age = 21;
    println!("My name is {} and I am {} years old", name, age);
    age = 30;
    println!("My name is {} and I am {} years old", name, age);

    // Define constant and always in upper case
    const ID: i32 = 001; // i32 is integer 32-bit 
    println!("ID: {}", ID);

    let ( my_name, my_age ) = ("Rishabh" , 21);
    println!("{} is {}", my_name, my_age);
}