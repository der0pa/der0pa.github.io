#![allow(unused_variables)]

use std::thread;
use std::time::Duration;

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calcutating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)    
        );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_closure(intensity)
                );
            }
        }
}


fn main() {
    println!("Hello, world!");
    
    // first closure below | arg1, arg2 | { body }
    let expensive_closure = |num: u32| -> u32 {
        println!("calcutation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // simulated_expensive_calculation(42);  // prints calculating slowly...
    generate_workout(24, 42);     // randon number is set to 3
}
