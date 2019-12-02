

fn main() { 

     let a = String::from("Go Heels");
     let c = a;
    println!("{:?}", a); 

    // raw pointers example
    let x = 5;
    let raw = &x as *const i32;
    //Error! Cannot dereference unless in unsafe mode
    // Prevents you from dereferencing at an unsafe location
    println!("raw points at {}", *raw);
 
     return; 
 } 




