use std::io;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {

    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("failed to read line");
    let n:u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("invalid input");
            return;
        }
    };

    let mut before:BigUint = Zero::zero();
    let mut after:BigUint = One::one();
    for _ in 0..=n {
        // println!("{}", before);

        let next = before + &after;
        before = after;
        after = next;
    }
    println!("{}", before);
}

