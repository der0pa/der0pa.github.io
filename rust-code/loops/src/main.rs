fn main() {
   let mut counter = 0;
   let result = loop {
       counter += 1;

       if counter == 10 {
           break counter * 2;
       }
    };

    let mut some_number = 3;
    while some_number != 0 {
        println!("{}", some_number);
        some_number -= 1;
    }

    println!("LIFTOFF!");
    println!("The result is {}", result);
    println!("Hello, world!");

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 4 {
        println!("the value is: {}", a[index] );
        index += 1;
    }
    
    for element in a.iter() {
        println!("the value is also: {}", element);
    }

    for countdown in (1..5).rev() {
        println!("{}!", countdown);   
    }
    println!("LIFTOFF!!!")

}
