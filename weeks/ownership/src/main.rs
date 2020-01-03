#![allow(unused_variables)]
fn main() {

    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    let t = String::from("hello world");
    let hello = &t[0..5];
    let world = &t[6..11];
    println!("{}", t);

    let x = 5;
    let y = x;
    println!("the value of x is: {} and y is: {}", x,y);

    let s1 = String::from("good day to you");
    let len = calculate_length(&s1);
    println!("length of '{}'is {},", s1, len);
    let s2 = s1.clone();
    println!("the value of s1: {} and s2: {}", s1, s2);

    let s = String::from("hello"); // s comes into scope here
    takes_ownership(s);  // s value moves into fn takes_ownership
    let x = 5;           // x comes into scope here
    makes_copy(x);       // x would move into fn makes_copy 
                    // but i32 has Copy trait so x can be used after }close bracket
                    // no heap mem used only stack mem.
  //  let z = s;  //  heap-value used after move wont compile

    let mut s3 = String::from("Hello") ;
    change(&mut s3);

    let mut r =String::from("hello world");
    let word = first_word(&r);
    r.clear();

}

fn takes_ownership(some_string: String) {  // some_string comes into scope here
    println!("{}", some_string);
} // some_string is now out of scope and 'drop' is called... memory freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope here
    println!("{}", some_integer);
}     // here some_integer goes out of scope. Nothing special happens
      // because i32 is 'Copy'  stack only memory is used

    // the ampersand sign '&' is used to make a 
    // reference to a value without taking ownership 
fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(q: &String) -> usize {
    let bytes = q.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    q.len()
}

