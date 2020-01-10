#![allow(unused_variables)]

fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("blue"), 10);
scores.insert(String::from("yellow"), 50);

let team_name = String::from("blue");
let score = scores.get(&team_name);

println!("{:?},{:?}", scores, score );

let text = "hello world wonderful world hello";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);

// this will cause a kernel `panic`
let v = vec![1, 2, 3];
println!("{:?}", v[2]);

} // end main()
