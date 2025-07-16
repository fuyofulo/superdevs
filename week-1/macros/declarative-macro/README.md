# Understanding Declarative Macros in Rust

## What are Declarative Macros?

Declarative macros (also known as "macros by example" or "macro_rules! macros") are a way to write code that writes other code in Rust. They allow you to write something similar to a match expression that operates on Rust code at compile time.

## Basic Syntax

Declarative macros are defined using `macro_rules!`. Here's a simple example:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    }
}

fn main() {
    say_hello!();  // Outputs: Hello!
}
```

## Common Built-in Declarative Macros

1. `println!` and `print!`
```rust
println!("Hello, world!");  // Basic usage
println!("Number: {}", 42); // With formatting
println!("{:?}", vec![1, 2, 3]); // Debug formatting
```

2. `vec!`
```rust
let v = vec![1, 2, 3];  // Creates a vector
```

3. `format!`
```rust
let message = format!("Hello, {}!", "Rust");
```

## Pattern Matching in Macros

Declarative macros can match different patterns:

```rust
macro_rules! say {
    // Match zero arguments
    () => {
        println!("Hello!");
    };
    
    // Match one argument
    ($msg:expr) => {
        println!("{}", $msg);
    };
    
    // Match multiple arguments
    ($msg:expr, $($arg:expr),*) => {
        println!($msg, $($arg),*);
    };
}

fn main() {
    say!();                    // Prints: Hello!
    say!("Hi there");         // Prints: Hi there
    say!("{} {}", "Hello", "World");  // Prints: Hello World
}
```

## Common Designators

When matching patterns in macros, you can use various designators:
- `expr`: for expressions
- `ident`: for identifiers
- `block`: for blocks of code
- `stmt`: for statements
- `ty`: for types
- `pat`: for patterns
- `path`: for paths
- `meta`: for meta items
- `tt`: for token trees

Example:
```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(hello);
create_function!(world);

fn main() {
    hello();  // Prints: You called "hello"()
    world();  // Prints: You called "world"()
}
```

## Repetition in Macros

You can use repetition patterns in macros:
- `*` for zero or more repetitions
- `+` for one or more repetitions
- `?` for zero or one repetition

```rust
macro_rules! make_list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = make_list![1, 2, 3, 4];  // Creates vec![1, 2, 3, 4]
}
```

## Best Practices

1. Use macros when you need to:
   - Generate repetitive code
   - Create domain-specific languages
   - Implement compile-time code generation

2. Avoid macros when:
   - Regular functions can do the job
   - The code becomes too complex to understand
   - You need runtime flexibility

3. Always document your macros well, as they can be harder to understand than regular code

## Debugging Macros

To see how a macro expands, you can use:
```bash
cargo expand
```
This shows you the actual Rust code that your macro generates.

## Key Points

1. Declarative macros are powerful tools for metaprogramming
2. They operate at compile time
3. They can reduce code duplication
4. They follow pattern matching syntax
5. They can accept variable numbers of arguments
6. Built-in macros like `println!` and `vec!` are commonly used examples
