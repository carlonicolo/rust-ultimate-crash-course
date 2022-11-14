fn main() {
    let mut bunnies = 3;
    bunnies = 5;
    println!("Hello, world! {}", bunnies);

    let x = 5;
    {
        let x = 23;
        println!("{}",x);
    }
    println!("{}",x);
}
