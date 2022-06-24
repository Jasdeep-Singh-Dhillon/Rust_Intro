// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Jasdeep";
    let mut age = 20;

    println!("My Name is {} and I am {}", name, age);

    age = 21;    
    println!("My Name is {} and I am {}", name, age);

    // Define constant
    const ID: i64 = 4294967270;
    println!("ID: {:x}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Jasdeep", 21);
    println!("{} is {}", my_name, my_age);
}