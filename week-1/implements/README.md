# Structs and Impl in Rust

We define our own combination of data types using a `struct`.

In our code, we have a `Rectangle` struct and a `Square` struct.

The `Rectangle` struct is a combination of two `u32` numbers named `width` and `height`:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

The `Square` struct is one `u32` number named `side`:

```rust
struct Square {
    side: u32,
}
```

Now, we need to define functions that these structs support. For this, we use `impl`, which means the struct implements the functions declared here.

For `Rectangle`:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

Similarly for `Square`:

```rust
impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}
```

To create an instance of the struct:

```rust
let rect1 = Rectangle {
    width: 10,
    height: 20,
};
```

For calling the functions, we pass this instance like this:

```rust
println!("The area of the rectangle is {}", rect1.area());
```

`rect1.area()` sends the `rect1` data to the `area` function defined in the `Rectangle` impl and returns a `u32` value which is the area of the rectangle.

Inside the area function, we are using `&self`. `&self` is an immutable reference to the struct, so this means we cannot change the values present in the struct. 

