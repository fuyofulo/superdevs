# Attribute Macros

Attribute macros are a powerful type of procedural macro in Rust that allow you to modify, extend, or transform code at compile time. Unlike derive macros that generate trait implementations, attribute macros can transform the items they're applied to, adding functionality, validation, or changing behavior.

## What are Attribute Macros?

Attribute macros are invoked using the `#[attribute_name]` syntax and can be applied to various Rust items like:
- Functions
- Structs and their fields
- Enums and their variants
- Modules
- Impl blocks

They receive the annotated item as input and can transform it or generate additional code.

## How They Work

1. **Compile-time transformation**: Attribute macros run during compilation
2. **Item modification**: They can modify, wrap, or extend the code they're applied to
3. **Code generation**: They can generate additional code alongside the original item
4. **Metadata processing**: They can read and process metadata to make decisions

## Serde: A Prime Example of Attribute Macros

This project demonstrates Serde, one of the most popular and powerful attribute macro systems in Rust for serialization and deserialization.

### Project Structure

The project showcases three main examples:

1. **Basic serialization/deserialization**
2. **Field-level attribute customization**
3. **Enum serialization**

## Code Examples Explained

### 1. Basic Serde Usage

```rust
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}
```

**What happens here:**
- `#[derive(Serialize, Deserialize)]` generates implementations for serialization traits
- The struct can be converted to/from JSON, MessagePack, YAML, etc.
- No additional configuration needed - uses sensible defaults

**Output:**
```json
{"name":"Alice","age":30}
```

### 2. Field-Level Attribute Customization

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename = "full_name")]
    name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    
    #[serde(skip)]
    password: String,
}
```

**Attribute explanations:**

#### `#[serde(rename = "full_name")]`
- **Purpose**: Changes the field name in serialized output
- **Use case**: API compatibility, following naming conventions
- **Result**: `name` field becomes `"full_name"` in JSON

#### `#[serde(skip_serializing_if = "Option::is_none")]`
- **Purpose**: Conditionally skip serialization based on a predicate
- **Use case**: Omit null/empty values from output
- **Result**: `email` field is omitted when `None`

#### `#[serde(skip)]`
- **Purpose**: Always skip this field during serialization/deserialization
- **Use case**: Sensitive data, computed fields, internal state
- **Result**: `password` never appears in serialized output

**Output:**
```json
{"full_name":"Bob"}
```
*Note: `email` is omitted (None), `password` is always skipped*

### 3. Enum Serialization

```rust
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,
    Inactive,
    Pending,
}
```

**What happens:**
- Simple enums serialize to strings by default
- Each variant becomes a string value
- Supports complex enums with data too

**Output:**
```json
"Active"
```

## Running the Examples

To see all examples in action:

```bash
cargo run
```

This will demonstrate:
1. Basic struct serialization and deserialization
2. Field attribute effects on JSON output
3. Enum serialization behavior

## Benefits of Attribute Macros

1. **Code Transformation**: Modify behavior without changing core logic
2. **Metadata-driven**: Configure behavior through attributes
3. **Composability**: Multiple attributes can work together
4. **Maintainability**: Centralized configuration through attributes
5. **Performance**: Compile-time code generation with no runtime overhead

## Best Practices

1. **Use established libraries**: Prefer well-tested attribute macros like Serde
2. **Document attributes**: Comment complex attribute configurations
3. **Test thoroughly**: Ensure attributes behave as expected
4. **Keep it simple**: Don't over-complicate with too many attributes
5. **Version carefully**: Attribute macro APIs can change between versions

## Key Differences from Derive Macros

| Aspect          | Derive Macros                    | Attribute Macros                      |
|-----------------|----------------------------------|---------------------------------------|
| **Syntax**      | `#[derive(Trait)]`               | `#[attribute_name]`                   |
| **Purpose**     | Generate trait implementations   | Transform/modify items                |
| **Flexibility** | Limited to traits                | Can modify any item                   |
| **Composition** | Multiple derives on one item     | Can nest and combine                  |
| **Scope**       | Whole item                       | Can target specific fields/variants   |

## Conclusion

Attribute macros are a powerful metaprogramming tool that enables:
- **Declarative programming**: Express what you want, not how to do it
- **Framework integration**: Seamlessly integrate with libraries and frameworks
- **Code generation**: Automatically generate boilerplate code
- **Behavior modification**: Change how code behaves without changing logic

The Serde example in this project demonstrates how attribute macros can provide fine-grained control over serialization behavior, making them essential for real-world Rust applications dealing with data interchange, APIs, and configuration management.
