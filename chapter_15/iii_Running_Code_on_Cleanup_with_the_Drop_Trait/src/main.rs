struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
     fn drop(&mut self) {
         println!("Dropping CustomSmartPointer with data `{}`!", self.data)
     }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("c::My name is Tapojit")
    };
    let d = CustomSmartPointer {
        data: String::from(("d::I am a PhD Student at UNT"))
    };
    println!("CustomSmartPointers created.");

    // lets drop 'c' manually
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}
