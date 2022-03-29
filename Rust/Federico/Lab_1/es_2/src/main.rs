use std::env;
use luhn::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{}", is_valid(&args[1]));
}