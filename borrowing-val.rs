fn main(){
let x: String = String::from("Hello ");
assert_eq!("Hello ",x);
let mut y: String = x;
y.push_str("World!");

println!("{}",y);
}