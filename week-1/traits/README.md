# Understanding Traits in Rust

Traits in Rust are similar to interfaces in other programming languages. They define a set of methods that types can implement.

## What is a Trait?

A trait tells the Rust compiler about functionality a type must provide. In our project, we define a `Shape` trait that any shape must implement:

```rust
trait Shape {
    fn whoami();
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}
```

This trait requires any implementing type to provide three methods:
- `whoami()`: A static method that identifies the shape
- `area(&self)`: Calculates the area of the shape
- `perimeter(&self)`: Calculates the perimeter of the shape

## Implementing Traits

We have two structs that implement the `Shape` trait:

### Rectangle

```rust
struct Rectangle {
    width: u32,
    height: u32
}

impl Shape for Rectangle {
    fn whoami() {
        println!("I am a rectangle");
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

### Square

```rust
struct Square {
    side: u32
}

impl Shape for Square {
    fn whoami() {
        println!("I am a square");
    }

    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}
```

## Using Traits

Here's how we use these implementations:

```rust
let rect1 = Rectangle {
    width: 10,
    height: 20
};

let square1 = Square {
    side: 10
};

// Using trait methods
Rectangle::whoami();  // Output: I am a rectangle
println!("Area of rectangle: {}", rect1.area());  // Output: 200
println!("Perimeter of rectangle: {}", rect1.perimeter());  // Output: 60

Square::whoami();  // Output: I am a square
println!("Area of square: {}", square1.area());  // Output: 100
println!("Perimeter of square: {}", square1.perimeter());  // Output: 40
```

## Key Points About Traits

1. Traits define shared behavior across different types
2. Each type implementing the trait must provide all required methods
3. Traits can have both instance methods (using `&self`) and static methods
4. Multiple types can implement the same trait with their own specific behavior
