#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    //Dropping a Value Early with std::mem::drop
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
//    println!("CustomSmartPointer created.");
    // Rust doesn’t let you call the trait’s method manually
//    c.drop();
//    println!("CustomSmartPointer dropped before the end of main.");

    let c = CustomSmartPointer {
        data: String::from("some data2"),
    };
    println!("CustomSmartPointer created2.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main2.");
}