#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 24, height: 42 };
    println!("rect1 isa' {:#?}", rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} // no semi-colon marks an expression. 


