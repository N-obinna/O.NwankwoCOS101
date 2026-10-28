fn main() {
    let v = vec![15, 25, 35, 45, 55];
    
    print_vector(v); // Moves v into the function
    
    // ERROR: v[0] fails because v was moved and dropped
    // println!("{}", v[0]); 
}

fn print_vector(x: Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}