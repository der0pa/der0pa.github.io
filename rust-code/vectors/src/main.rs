#![allow(unused_variables)]

fn main() {

    // let v: Vec<i32> = Vec::new();  or
    // `vec!` macro creates a vector
    let mut v = vec![1,2,3];  
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(42);

    println!("after pushing above: {:?}",v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut v = vec![100, 32, 57];  // must be shadowing above `v`
    for i in &mut v {
        *i += 50;  // `*` is dereference operator
        println!("{}", i);
    }
    println!("{:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // println!("{}",rows);

}   // closing bracket of fn main()

