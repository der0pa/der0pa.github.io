fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let z = plus_one(41);
    println!("The value of z is: {}", z);

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("a[index] is {}", a[index]);
        index += 1;
    }

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
fn plus_one(z: i32) -> i32 {
    z + 1
}

