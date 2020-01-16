#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {        // this is a method when inside an `impl`
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };  

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));     
}


/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} // no semi-colon marks an expression. 

output: 
    Finished dev [unoptimized + debuginfo] target(s) in 2.15s
     Running `target/debug/rectangles`
can rect1 hold rect2? true
can rect1 hold rect3? false


*/

