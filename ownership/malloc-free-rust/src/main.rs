// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn main() {

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    /// here we are moving the value of a into b 
    let b = a;

    /// a can no longer access this memory because it no longer owns the heap memory
    println!("a contains: {}", a);

    /// This function takes ownership of the heap allocated memory from `b`
    /// the heap memory used to hold b is now freed
    destroy_box(b);
    println!("b contains: {}", b);
}
