fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; 
        *y = 6; 
    } // The mutable reference `y` is dropped here
    { // Create another new scope
        let z = &mut x;
        *z = 7; 
    } // The mutable reference `z` is dropped here
    println!("x = {}", x); // This will print 7
}
