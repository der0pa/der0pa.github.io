fn main() {
    let number = 6;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    } 

    let condition = true;  //false;
    let digit = if condition {
        5
    } else {
        6
    };
    println!("the value of digit is: {}", digit);

}