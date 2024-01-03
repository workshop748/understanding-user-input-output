fn main() {
use std::io;
let mut name =String::new();
println!("what is your name?");
io::stdin().read_line(&mut name).unwrap();
println!("hi, {name}!");
}
