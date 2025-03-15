fn main() {
    let s = String::new();

    let data = "initial contents"; // this is called string literals. it is immutable
    println!("{}", data);
    let s2 = data.to_string();
    let s3 = String::from("YES");
    println!("{s3}");

    hello_around_the_world();

    let mut s = String::from("foo");
    let s3 = "bar";
    s.push_str(s3);
    println!("{s}  {s3}");

    s.push('!'); // push() --> pushes a single character
    println!("{s}");


    concatenation();
    using_index();
    iterate_over_strings();
}

fn hello_around_the_world() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    let hello = String::from("Hola");

    println!("{}", hello);
}

fn concatenation() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // fn add(self, s: &str) --> this is the signature of the '+' operator
    // the s2 will be added to s1, then the ownership of s1 will be transferred to s3

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    // it is cumbersome to read this code, so lets use format
    println!("{s}");

    let s1 = String::from("tic");

    let s3 = format!("{s1}-{s2}-{s3}"); // no ownership is transferred
}

fn using_index() {
    let hello = "Здравствуйте";
    let answer = &hello[0..2];

    println!("{answer}");
}

fn iterate_over_strings() {
    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("{c} ");
    }
    println!();
    for b in hello.bytes() {
        print!("{b} ")
    }
}