fn main() {
    println!("Hello Tapojit!");
    variable();
    define_x();
    shadowing();
    tuple();
    destructure();
}

fn variable() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("The value of x is {}", x);
    println!("Success!");
}

fn define_x() {
    let x = "hello";
    println!("{}, world!", x)
}

fn shadowing() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // This will make the function panic, if x != 12
    }
    assert_eq!(x, 5);

    let x = 42; // The previous value is shadowed
    println!("{}", x);

    // Shadowing can be done for other types as well
    let x = "HELLO RUST";
    println!("{}", x);

    let _a = 1;
}

fn tuple() {
    let (mut x, mut y) = (1, 2);
    x += 1;
    y += 1;

    assert_eq!(x, 2);
    assert_eq!(y, 3);
    println!("Success!");
}

fn destructure() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x,y], [3,2]);
    println!("Success!");
}