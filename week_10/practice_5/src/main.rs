fn main() {
    let x = vec![100, 200, 300];
    
    // Passing a reference (&x) instead of ownership
    borrow_vector(&x); 
    
    // This works fine because x was only borrowed, not moved!
    println!("Printing the value from main() x[0]={}", x[0]);
}

fn borrow_vector(z: &Vec<i32>) {
    println!("Inside borrow_vector function {:?}", z);
}