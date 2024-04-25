fn main() {
    let mut hello_world = String::from("Unchanged string var");
    println!("{}", hello_world);
    hello_world = String::from("Hello, world!");
    println!("{}", hello_world);
    let mut bananas = 0;
    println!("{}", bananas);
    bananas = 10;
    println!("{}", bananas);
}
