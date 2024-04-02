use std::io;
use std::any::TypeId;
fn main() {
    println!("Computational Geometry - Praktikum 1");

    println!("Hello World!");

    let mut var1 = String::new();
    io::stdin()
        .read_line(&mut var1)
        .expect("Error reading line...");
    let var2: f32 = "143.3".parse().expect("NaN!");
    println!("{}", var2);

    println!("You typed: {}", var1);
    

}
