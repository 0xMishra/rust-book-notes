use std::slice;

// In Rust, global variables are called static variables. E.g.:
static HELLO_WORLD: &str = "Hello, world!";

pub fn run_unsafe_rust() {
    //  We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // this might or might not be invalid
    let address = 0x012345usize;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe {
        dangerous();
    }

    // creating a safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using extern function to call external code
    // Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has the keyword extern
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
unsafe fn dangerous() {}

// creating a safe abstraction over unsafe code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// mutating a static variable is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
