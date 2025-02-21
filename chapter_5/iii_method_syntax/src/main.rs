#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// associated functions
impl Rectangle {
    // method: it takes the object as an parameter
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // method
    fn width(&self) -> bool {
        self.width > 0
    }
    // method
    fn height(&self) -> bool {
        self.height > 0
    }

    // method
    // Returns if this rectangle can hold the given rectangle
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height >= rect.height && self.width >= rect.width
    }

    // not a method
    // because does not take the object as a parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 10
    };

    if rect1.height() && rect1.width() {
        println!("It is a valid rectangle with sides equal to {} and {} pixels", rect1.width, rect1.height);
    } else {
        println!("It is an invalid rectangle.");
        return
    }

    println!("The area of the rectagle is {} square pixels.", rect1.area());

    println!("Can rect1 hold rect2? --> {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? --> {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect3? --> {}", rect1.can_hold(&rect1));

    let sq = Rectangle::square(30);
    println!("{:?}", sq);
}
