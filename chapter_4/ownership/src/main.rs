fn main() {
    let s = "Hello World";
    // s = "100"; This can not be done
    println!("{s}");

    let mut mut_str = String::from("Hello World");
    mut_str.push_str("!!!");
    println!("{mut_str}"); 

    let copy_mut_str = mut_str;
    println!("{copy_mut_str}");
    // println!("{mut_str}");  We cannot do this as mut_str is moved to copy_mut_str

    // Now if we want to make a completely new copy of mut_str,
    // we should do like the following:
    let copy_copy_mut_str = copy_mut_str.clone();
    println!("{copy_mut_str}");

    let l = takes_ownership(copy_copy_mut_str);
    // After sending a variable to a function,
    // it is moved or the ownership is moved
    // and we can no longer access it after that

    // println!(copy_copy_mut_str); // This is wrong

    let s2 = takes_and_gives_back_ownership(copy_mut_str);
    println!("takes and gives back ownership: {s2}");
    // println!("{copy_mut_str}"); --> We can not do this, because the ownership is moved
    // So the solution to this problem is using reference
}


fn takes_ownership(s: String) -> usize {
    s.len()
}

fn takes_and_gives_back_ownership(s: String) -> String {
    s
}