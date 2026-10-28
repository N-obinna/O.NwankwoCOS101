fn main() {
    let v = vec![10, 20, 30];
    let v2 = v;
    
    // v2 moves into the function, so it is invalidated in main
    display(v2);
    
    // ERROR: v2 cannot be used here anymore
    // println!("In main {:?}", v2);
}

fn display(v: Vec<i32>) {
    println!("inside display {:?}", v);
}