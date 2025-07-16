use borsh::{BorshSerialize, BorshDeserialize, to_vec, from_slice};

// Example 1: BorshSerialize and BorshDeserialize
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    name: String,
    age: u32,
    pub_key: Vec<u8>, // Using Vec<u8> instead of PubKey for simplicity
}

// Example 2: Debug trait
#[derive(Debug)]
struct DebugUser {
    name: String,
    num: u32,
}

// Example 3: Copy and Clone traits
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Example 4: Clone trait (for types that can't implement Copy)
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

// Example 5: Multiple derives working together
#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

fn main() {
    println!("=== Custom Derive Macros Examples ===\n");
    
    // Example 1: BorshSerialize and BorshDeserialize
    println!("1. BorshSerialize and BorshDeserialize:");
    let user = User {
        name: String::from("Alice"),
        age: 30,
        pub_key: vec![1, 2, 3, 4, 5],
    };
    
    println!("Original user: {:?}", user);
    
    // Serialize to bytes
    let serialized = to_vec(&user).unwrap();
    println!("Serialized bytes: {:?}", serialized);
    
    // Deserialize back to struct
    let deserialized: User = from_slice(&serialized).unwrap();
    println!("Deserialized user: {:?}", deserialized);
    println!();
    
    // Example 2: Debug trait
    println!("2. Debug trait:");
    let debug_user = DebugUser {
        name: String::from("Bob"),
        num: 42,
    };
    println!("Debug output: {:?}", debug_user);
    println!();
    
    // Example 3: Copy and Clone traits
    println!("3. Copy and Clone traits:");
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 is copied, not moved - p1 is still usable
    println!("Original point p1: {:?}", p1);
    println!("Copied point p2: {:?}", p2);
    println!("p1 is still usable after 'move': {:?}", p1);
    
    // Explicit cloning
    let p3 = p1.clone();
    println!("Explicitly cloned point p3: {:?}", p3);
    println!();
    
    // Example 4: Clone trait for non-Copy types
    println!("4. Clone trait for non-Copy types:");
    let person1 = Person {
        name: "Charlie".to_string(),
        age: 25,
    };
    let person2 = person1.clone(); // Explicit cloning required
    println!("Original person: {:?}", person1);
    println!("Cloned person: {:?}", person2);
    println!();
    
    // Example 5: Multiple derives
    println!("5. Multiple derives (Debug, Clone, PartialEq):");
    let product1 = Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 999.99,
    };
    let product2 = product1.clone();
    let product3 = Product {
        id: 2,
        name: "Mouse".to_string(),
        price: 25.50,
    };
    
    println!("Product 1: {:?}", product1);
    println!("Product 2 (cloned): {:?}", product2);
    println!("Product 3: {:?}", product3);
    println!("Product 1 == Product 2: {}", product1 == product2);
    println!("Product 1 == Product 3: {}", product1 == product3);
    println!();
    
    // Demonstrating the difference between Copy and Clone
    println!("=== Copy vs Clone Demonstration ===");
    demonstrate_copy_vs_clone();
}

fn demonstrate_copy_vs_clone() {
    // Copy types: implicit duplication
    let num1: i32 = 42;
    let num2 = num1; // Copy happens automatically
    println!("num1: {}, num2: {}", num1, num2); // Both are usable
    
    // Point implements Copy, so assignment copies
    let point1 = Point { x: 10, y: 20 };
    let point2 = point1; // Copy happens automatically
    println!("point1: {:?}, point2: {:?}", point1, point2); // Both are usable
    
    // Person doesn't implement Copy, so we need explicit cloning
    let person1 = Person {
        name: "Dave".to_string(),
        age: 35,
    };
    // let person2 = person1; // This would move person1, making it unusable
    let person2 = person1.clone(); // Explicit clone required
    println!("person1: {:?}, person2: {:?}", person1, person2); // Both are usable
}
