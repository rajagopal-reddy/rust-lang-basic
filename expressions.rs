fn main() {
    let x: i32 = 2;
    let z: i32 = 4;
    
    let y: i32 = {
        let mut y: i32 = 2;
        
        y += z; 
        y 
    };

    fn add(x: i32, y: i32, z: i32) -> i32 {
        x + y + z 
    }
    println!("{} is the total value",add(x,y,z));
}