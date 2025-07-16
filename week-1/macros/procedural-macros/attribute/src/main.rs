
use serde::{Deserialize, Serialize};

// Basic serde example
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

// Serde with field attributes
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename = "full_name")]
    name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    
    #[serde(skip)]
    password: String,
}

// Simple enum
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
}

fn main() -> Result<(), serde_json::Error> {
    // Basic usage
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let json = serde_json::to_string(&user)?;
    println!("JSON: {}", json);
    // Output: {"name":"Alice","age":30}
    
    let back: User = serde_json::from_str(&json)?;
    println!("Back to struct: {:?}", back);
    
    // With attributes
    let person = Person {
        name: "Bob".to_string(),
        email: None,
        password: "secret".to_string(),
    };
    
    let json = serde_json::to_string(&person)?;
    println!("Person JSON: {}", json);
    // Output: {"full_name":"Bob"} 
    // Note: email is None so skipped, password is always skipped
    
    let status = Status::Active;
    let status_json = serde_json::to_string(&status)?;
    println!("Status: {}", status_json);
    // Output: "Active"
    
    Ok(())
}