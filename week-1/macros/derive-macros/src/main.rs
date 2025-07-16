#[derive(Debug)]
struct User {
    name: String,
    age: u32
}

use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let user1 = User {
        name: String::from("fuyo"),
        age: 21
    };

    // Using Debug formatting
    println!("Debug format: {:?}", user1);
    
    // Using Display formatting
    println!("Display format: {}", user1);
    
    // Using Pretty Debug formatting
    println!("Pretty debug format: {:#?}", user1);
}
