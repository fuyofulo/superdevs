struct Rectangle {
    width: u32,
    height: u32
}

struct Square {
    side: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height)
    }
}

impl Square {
    fn area(&self) -> u32 {
        return self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        return 4 * self.side
    }
}



fn main() {
    
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let square1 = Square {
        side: 10
    };

    println!("the area of the rectangle is {}", rect1.area());
    println!("the perimeter of the rectangle is {}", rect1.perimeter());
    println!("the area of the square is {}", square1.area());
    println!("the perimeter of the square is {}", square1.perimeter());
}
