fn main() {
    another_function(5, 6);    
    
    let x = 5;
    
    let y = {
        let x = 3;
        x + 2        // no semi-colon is used for an expression
    };

    println!("3the value of x is: {}", x);
    println!("4the value of y is: {}", y);

    let x = five();
    println!("5the value of x is: {}", x);

    let x = plus_one(8);
    println!("6the value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("1the value of x is: {}", x);
    println!("2the value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
