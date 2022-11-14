use functions::greet;

fn main() {
    println!("Hello, world!");
    let x = do_stuff(2.0, 12.5);
    println!("This is the result -> {}",x);

    greet();
}

fn do_stuff(quantity: f64, oz: f64) -> f64 {
    quantity * oz
}