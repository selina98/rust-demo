// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn main() {

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    // ownership of the data in a is now owned by b
    let b = a;

    // a can no longer access this memory
    // TODO 
    //println!("a contains: {}", a);

    // This function takes ownership of the heap allocated memory from `b`
    // the heap memory used to hold b is now freed
    // TODO
    destroy_box(b);
    //println!("b contains: {}", b);
}
