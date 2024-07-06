// Listing 19-9: Defining and using an immutable static variable
static HELLO_WORLD: &str = "Hello, world!";

// Listing 19-10: Reading from or writing to a mutable static variable
// is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main () {
    // Listing 19-1: Creating raw pointers from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Listing 19-3: Dereferencing raw pointers within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Listing 19-2: Creating a raw pointer to an arbitrary memory
    // address
    let address = 0x012345usize;
    let r = address as *const i32;
    
    unsafe {
        dangerous();
    }

    // Listing 19-4: Using the safe split_at_mut function
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    
    println!("name is: {HELLO_WORLD}");

    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

unsafe fn dangerous() {}

// Listing 19-6: Using unsafe code in the implementation of the
// split_at_mut function
use std::slice;

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

// Listing 19-8: Declaring and calling an extern function defined in
// another language
extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust functino from C!");
}

// Listing 19-11: Defining and implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}