#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct DifferentPoint<T, U> {
    x: T,
    y: U,
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// Generic type parameters in a struct definition aren’t always
// the same as those you use in that same struct’s method signatures.
impl <X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn struct_main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 2.0};

    println!("integer point: {:?}", integer);
    println!("float point {:?}", float);


    let both_integer = DifferentPoint { x: 5, y: 10 };
    let both_float = DifferentPoint { x: 1.0, y: 2.0 };
    let integer_and_float = DifferentPoint { x: 5, y: 10.5 };

    println!("both_integer: {:?}", both_integer);
    println!("both_float: {:?}", both_float);
    println!("integer_and_float: {:?}", integer_and_float);

    println!("x value of integer: {}", integer.x());
    println!("distance from origin of float point: {}", float.distance_from_origin());
    // println!("distance from origin of integer point: {}", integer.distance_from_origin());
    // This won't work because the distance_from_origin() function is
    // implemented only for 'float' type

    let p1 = Point2{x: 5, y: 10.4};
    let p2 = Point2{x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

