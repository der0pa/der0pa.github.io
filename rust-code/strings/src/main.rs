fn main() {
let data = "initial content";
let s = data.to_string();

println!("{}", s);

let s = "initial contents".to_string();
println!("{}", s);

let hello = String::from("السلام عليكم");
let _hello = String::from("שָׁלוֹם");
println!("{}", _hello);
println!("{}", hello);

let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("the value of s2: {}", s2);

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
println!("{}", s);

let hello = "Здравствуйте";
let s = &hello[0..4];
println!("{}",s);



} // close main()
