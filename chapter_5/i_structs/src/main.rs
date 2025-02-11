struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("tapojit047"),
        email: String::from("paultapojit@gmail.com"),
        sign_in_count: 1,
    };

    let mut mutable_user = User {
        active: true,
        username: String::from("hellyeah"),
        email: String::from("hellyeah@gmail.com"),
        sign_in_count: 10,
    };

    // A whole instance of struct can be mutable
    // However, it is impossible to make a particular 
    // field mutable 

    println!("{}", user1.sign_in_count);
    println!("{}", mutable_user.email);

    mutable_user.email = String::from("hellno@gmail.com");
    println!("{}", mutable_user.email);


    let new_user = build_user(String::from("abc@gmail.com"), String::from("abc"));
    println!("{}", new_user.email);

    // Copying from another instance
    let copy_user = User {
        email: String::from("copy_user@gmail.com"),
        ..user1
    };
    println!("{} {}", copy_user.email, copy_user.username);
    //  In this example, we can no longer use user1 as a whole after creating user2 
    // because the String in the username field of user1 was moved into user2. 
    // If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1,
    // then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait

    // println!("{}", user1.username); // invalid
    println!("{}, {}, {}", user1.email, user1.active, user1.sign_in_count); // valid



    // Using Tuple Structs Without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 10, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.1);

    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 100,
    }
}
