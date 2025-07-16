# Custom Derive Macros

Custom derive macros are a powerful feature in Rust that automatically generate trait implementations for your structs and enums. They save you from writing boilerplate code by automatically implementing common traits based on the structure of your data.

## What are Derive Macros?

Derive macros are procedural macros that generate code at compile time. When you use `#[derive(TraitName)]` on a struct or enum, the macro analyzes the structure and generates an implementation of that trait.

## How They Work

1. **Compile-time code generation**: Derive macros run during compilation
2. **Automatic trait implementation**: They generate trait implementations based on your data structure
3. **No runtime cost**: All code generation happens at compile time

## Examples in This Project

This project demonstrates several important derive macros:

### 1. BorshSerialize and BorshDeserialize

The Borsh (Binary Object Representation Serializer for Hashing) derive macros enable efficient serialization and deserialization:

```rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    name: String,
    age: u32,
    pub_key: Vec<u8>,
}
```

**What it generates:**
- `BorshSerialize`: Converts the struct to a byte array for storage/transmission
- `BorshDeserialize`: Reconstructs the struct from a byte array

**Use cases:**
- Blockchain applications
- Network protocols
- Data persistence
- Cross-language data exchange

### 2. Debug Trait

The Debug trait enables formatted output using `{:?}`:

```rust
#[derive(Debug)]
struct DebugUser {
    name: String,
    num: u32,
}
```

**What it generates:**
- Implementation of `std::fmt::Debug`
- Enables `println!("{:?}", instance)` for debugging

**Use cases:**
- Development and debugging
- Error messages
- Logging

### 3. Copy and Clone Traits

These traits control how values are duplicated:

#### Copy Trait
```rust
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

**What Copy provides:**
- **Implicit duplication**: Values are automatically copied when moved
- **Bit-level copying**: Fast, simple memory copy (memcpy)
- **No custom logic**: Cannot define custom copy behavior
- **Stack-only types**: Only works with types that don't allocate memory

**Key behavior:**
```rust
let p1 = Point { x: 1, y: 2 };
let p2 = p1; // p1 is copied, not moved - p1 is still usable
```

#### Clone Trait
```rust
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}
```

**What Clone provides:**
- **Explicit duplication**: Must call `.clone()` to duplicate
- **Custom logic allowed**: Can define complex cloning behavior
- **Works with heap types**: Handles types that allocate memory (like String)
- **More expensive**: Can perform complex operations

**Key behavior:**
```rust
let p1 = Person { name: "Alice".to_string(), age: 30 };
let p2 = p1.clone(); // Explicit cloning required
```

### 4. Multiple Derives

You can combine multiple derives on a single type:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}
```

This generates implementations for:
- `Debug`: For formatted output
- `Clone`: For explicit duplication
- `PartialEq`: For equality comparison (`==` and `!=`)

### 5. Copy vs Clone: Key Differences

| Aspect | Copy | Clone |
|--------|------|-------|
| **Invocation** | Automatic (implicit) | Manual (explicit `.clone()`) |
| **Performance** | Very fast (memcpy) | Can be expensive |
| **Memory** | Stack types only | Stack and heap types |
| **Customization** | No custom logic | Can define custom behavior |
| **Types** | `i32`, `f64`, `bool`, arrays | `String`, `Vec`, custom types |

## Running the Examples

To see all examples in action:

```bash
cargo run
```

This will demonstrate:
1. Serialization/deserialization with Borsh
2. Debug output formatting
3. Copy vs Clone behavior differences
4. Multiple trait implementations working together

## Key Benefits of Derive Macros

1. **Reduced boilerplate**: No need to manually implement common traits
2. **Consistency**: Generated implementations follow standard patterns
3. **Maintainability**: Automatically updates when you modify struct fields
4. **Type safety**: Compile-time generation ensures type correctness
5. **Performance**: No runtime overhead

## Common Derive Macros

- `Debug`: Formatted output for debugging
- `Clone`: Explicit duplication
- `Copy`: Implicit duplication (for simple types)
- `PartialEq`/`Eq`: Equality comparison
- `PartialOrd`/`Ord`: Ordering comparison
- `Hash`: Hashing for use in HashMap/HashSet
- `Default`: Default value construction
- `Serialize`/`Deserialize`: Serde serialization (most popular)
- `BorshSerialize`/`BorshDeserialize`: Borsh serialization (blockchain-focused)

## Best Practices

1. **Always derive `Debug`** for structs during development
2. **Use `Copy` for simple, stack-only types** (numbers, booleans)
3. **Use `Clone` for complex types** that allocate memory
4. **Combine derives** when you need multiple traits
5. **Consider serialization needs** (Serde vs Borsh vs others)
6. **Test your derives** to ensure they work as expected

The derive macro system is one of Rust's most powerful features, enabling clean, maintainable code while preserving performance and type safety.
