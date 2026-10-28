struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method using &self to borrow the instance
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let small = Rectangle {
        width: 10,
        height: 20,
    };
    
    println!("width is {} \nheight is {} \narea of Rectangle is {}", small.width, small.height, small.area());
}