use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::fs;
use std::error::Error;

fn      get_coeff(cont: String) -> (f64, f64) {
    let ret:Vec<f64> = cont.split(',')
        .map(|s| s.parse().unwrap()).collect();
    return (ret[0], ret[1]);
}

fn main() {
    println!("Please input your milleage.");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Incorrect input"),
    };

    let mut coeff = (0.0, 0.0);
    if fs::metadata("theta.csv").is_ok() {
        let mut file = match File::open("theta.csv") {
            Err(why) => panic!("Could not read because {}", Error::description(&why)),
            Ok(file) => file,
        };

        let mut s = String::new();
        coeff = match file.read_to_string(&mut s) {
            Err(why) => panic!("Could not read because {}", Error::description(&why)),
            Ok(_) => get_coeff(s),  // Use a regex here for input validation.
        };
    }

    if (coeff.0 + coeff.1 * input as f64).is_nan() {
        println!("Price cannot be displayed");
    } else {
        println!("Car cost around {} money", (coeff.0 + coeff.1 * input as f64) as i64);
    }
}
