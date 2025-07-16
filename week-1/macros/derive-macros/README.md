# Understanding Derive Macros in Rust

## What are Derive Macros?

Derive macros in Rust are a powerful feature that allows automatic implementation of traits for custom types. They are used with the `#[derive()]` attribute syntax and can significantly reduce boilerplate code.

When you add `#[derive(TraitName)]` above a struct or enum:
1. The compiler looks for an implementation of that derivable trait
2. It automatically generates the appropriate implementation for your type
3. This generated code follows a standard pattern based on the trait's requirements

## Debug vs Display Traits

### Debug Trait (`std::fmt::Debug`)
- Can be automatically derived using `#[derive(Debug)]`
- Meant for developers and debugging purposes
- Provides a default implementation that shows struct fields and their values
- Example output: `User { name: "fuyo", age: 21 }`
- Used with `{:?}` or `{:#?}` (pretty print) format specifier

### Display Trait (`std::fmt::Display`)
- Cannot be automatically derived
- Meant for end-users
- Requires manual implementation
- Allows custom formatting for user-friendly output
- Used with `{}` format specifier

## Implementation Example

Here's how we implement both traits for a `User` struct:

```rust
#[derive(Debug)]  // Automatic implementation
struct User {
    name: String,
    age: u32
}

// Manual implementation of Display
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}
```

## Using the Traits

```rust
let user1 = User {
    name: String::from("fuyo"),
    age: 21
};

// Different ways to print
println!("Debug format: {:?}", user1);      // User { name: "fuyo", age: 21 }
println!("Display format: {}", user1);      // fuyo is 21 years old
println!("Pretty debug format: {:#?}", user1);  // Nicely formatted multi-line output
```

## Why Can't Display be Derived?

The reason `Display` cannot be derived automatically is that it's meant for user-facing output. Rust can't make assumptions about how you want to present your data to users. For example:
- Should a `Person` be displayed as "First Last" or "Last, First"?
- Should a `Point` be shown as "(x,y)" or "x: _, y: _"?
- Should measurements include units?

These decisions require developer input, which is why `Display` must be implemented manually.

## Key Points

1. Use `Debug` (`{:?}`) during development and debugging
2. Use `Display` (`{}`) for user-facing output
3. `Debug` can be automatically derived
4. `Display` requires thoughtful manual implementation
5. You can implement both traits for the same type
6. Pretty printing with `{:#?}` provides nicely formatted debug output