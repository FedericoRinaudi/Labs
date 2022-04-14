use binary_search::*;
use std::env;
use std::{io, io::prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut v = vec![];
    for line in io::stdin().lock().lines() {



        //println!("s: {}", line.unwrap());
        let el = line.unwrap().parse::<i32>();
        match el {
            Ok(r)=> {
                v.push(r);
            }
            _ => {
            }
        }
    }
    v.sort();
    let res = find(v, args[1].parse::<i32>().expect("Input not numeric"));
    match res {
        Some(x) => {
            println!("{} at index: {}", args[1].parse::<i32>().unwrap(), x);
        },
        None => {
            println!("{} not found", args[1].parse::<i32>().unwrap());
        }
    }

}
