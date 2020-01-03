fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let z = plus_one(41);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
fn plus_one(z: i32) -> i32 {
    z + 1
}
