# Understanding Rust Macros

Rust has two main types of macros:
1. Declarative Macros (`macro_rules!`)
2. Procedural Macros

## 1. Declarative Macros
These are simpler macros created using `macro_rules!`. They follow a pattern-matching style syntax.
[See detailed explanation in declarative-macro folder](./declarative-macro/README.md)

## 2. Procedural Macros

Procedural macros are more powerful than declarative macros because they give you full control over the input tokens and allow you to generate arbitrary Rust code as output. There are three types of procedural macros:

### a. Derive Macros
- Used with the `#[derive(MacroName)]` syntax
- Operate on struct/enum definitions
- Generate additional code based on the annotated type
- Example: `#[derive(Debug)]`, `#[derive(Clone)]`
[See implementation in derive-macros folder](./derive-macros/README.md)

### b. Attribute Macros
- Used as attributes with `#[macro_name]` syntax
- Can be applied to any item (functions, structs, modules, etc.)
- Can modify the item they're attached to and generate additional code
- Example: `#[test]`, `#[route(GET, "/")]`

### c. Function-like Macros
- Look like function calls but run at compile time
- Can take any kind of input tokens and produce any output
- Example: `sql!(SELECT * FROM users)`

## Project Structure

```
macros/
├── README.md (this file)
├── declarative-macro/    # Example of macro_rules! macros
├── derive-macros/       # Example of custom derive macros
├── attribute-macros/    # Example of attribute macros
└── function-macros/     # Example of function-like macros
```

## Key Differences

1. **Declarative Macros**
   - Pattern-matching based
   - Simpler to write
   - Limited in functionality
   - Good for simple code generation

2. **Procedural Macros**
   - Full programmatic control
   - More powerful and flexible
   - Require separate crate
   - Can manipulate Rust's syntax tree

## Setting Up Procedural Macros

All procedural macros must be in their own crate with the following in `Cargo.toml`:
```toml
[lib]
proc-macro = true
```

Common dependencies for procedural macros:
```toml
[dependencies]
syn = "2.0"      # For parsing Rust code
quote = "1.0"    # For generating Rust code
proc-macro2 = "1.0"  # Lower-level API for proc macros
```

## When to Use Each Type

1. **Derive Macros**: When you want to automatically implement traits for structs/enums
   ```rust
   #[derive(MyCustomTrait)]
   struct MyStruct {}
   ```

2. **Attribute Macros**: When you want to add metadata or modify items
   ```rust
   #[route(GET, "/users")]
   fn get_users() {}
   ```

3. **Function-like Macros**: When you need to generate code from arbitrary input
   ```rust
   let sql = sql!(SELECT * FROM users WHERE id = 1);
   ```

## Best Practices

1. **Use Declarative Macros When:**
   - Pattern matching is sufficient
   - You need simple token substitution
   - The macro rules are straightforward

2. **Use Procedural Macros When:**
   - You need complex code generation
   - You want to parse Rust syntax
   - You need to implement traits automatically
   - You want to create domain-specific languages

3. **General Guidelines:**
   - Document your macros thoroughly
   - Provide clear error messages
   - Keep the generated code readable
   - Test both success and error cases 