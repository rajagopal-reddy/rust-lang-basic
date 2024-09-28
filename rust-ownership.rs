fn main() {
    let x: String = String::from("Hello World!");
    let y: String = modify(x); 
    assert_eq!(y, "Hello World!");
}

fn modify(x: String) -> String {
    println!("{}", x);
    x
}