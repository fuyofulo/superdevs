#[derive(Debug)] // Debug is a custom derive proc macro

struct User {
   name: String,
   age: u32
}

fn main() {
   println!("hi there");  // declarative macro 
}

#[post("/user/")]  // attribute like proc macro
fn create_user() {
   sqlx::query!("INSERT INTO USER VALUES ()")  // function like proc macro
}