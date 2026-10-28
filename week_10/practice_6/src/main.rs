fn add_one(e: &mut i32) {
    *e += 1; // Dereference and modify original value
}

fn main() {
    let mut i = 3;
    
    add_one(&mut i); // Borrow mutably
    
    println!("{}", i); // Prints 4
}