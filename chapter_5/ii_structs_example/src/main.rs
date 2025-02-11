#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: dbg!(30 * 2),
        width: 50,
    };

    let area = area(&rect1);
    println!("The area of the rectange is {area} square pixels.");


    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}