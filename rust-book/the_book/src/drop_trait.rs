// Drop lets you customize what happens when a value is about to go out of scope.
use std::mem::drop;
pub fn run_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // here Rust will call the code we put in the drop method, printing our final message.We don’t need to call the drop method explicitly.

    //  we can’t call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use the std::mem::drop function
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}

// Drop trait implementation
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // drop function
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
    //  Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main
}

// A destructor is analogous to a constructor, which creates an instance. The drop function in Rust is one particular destructor.
