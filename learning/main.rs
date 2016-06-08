use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;

fn      get_coeff(cont: String) -> Vec<(i64, i64)> {
    let mut coeff: Vec<(i64, i64)> = Vec::new();
    let mut list = cont.split('\n');
    list.next();

    for line in list {
        if line.len() < 1 {
            continue;
        }
        let tmp: Vec<&str> = line.split(',').collect();
        let mut value: (i64, i64) = (0, 0);

        value.0 = match tmp[0].trim().parse() {
            Err(_) => panic!("Can not parse file"),
            Ok(flo) => flo,
        };
        value.1 = match tmp[1].trim().parse() {
            Err(_) => panic!("Can not parse file"),
            Ok(flo) => flo,
        };
        coeff.push((value.0, value.1));
    }
    return coeff;
}

fn      iter(coeff: Vec<(i64, i64)>, max: i32) -> (f64, f64) {
    let mut theta = (0.0, 0.0);
    let mut sum = (0.0, 0.0);
    let     rate:f64 = 0.01 / coeff.len() as f64;

    for c in 0..max {
        for val in &coeff {
            sum.0 += theta.0 + (theta.1 * val.0 as f64 / 10000.0) - val.1 as f64;
            sum.1 += ((theta.0 + (theta.1 * val.0 as f64 / 10000.0)) - val.1 as f64) * val.0 as f64 / 10000.0;
        }
        if c % 50 == 0{
            println!("{}\t{}", sum.0, sum.1);
        }
        theta.0 -= rate * sum.0;
        theta.1 -= rate * sum.1;
        sum.0 = 0.0;
        sum.1 = 0.0;
    }
    return (theta.0, theta.1 / 10000.0);
}

fn      main() {
    let instant : std::time::Instant = std::time::Instant::now();
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 || args.len() == 3 {
        let path = Path::new(&args[1]);

        let mut file = match File::open(&path) {
            Err(why) => panic!("Can not open {} because {}", path.display(), Error::description(&why)),
            Ok(file) =>file,
        };

        let mut iteration: i32 = 5500;
        if args.len() == 3 {
            iteration = match args[2].trim().parse() {
                Err(_) => panic!("Custom value of iteration is incorrect"),
                Ok(nb) => nb,
            };
        }

        let mut s = String::new();
        let coeff = match file.read_to_string(&mut s) {
            Err(why) => panic!("Could not read {} because {}", path.display(), Error::description(&why)),
            Ok(_) => get_coeff(s),
        };

        let val = iter(coeff, iteration);

        file = match File::create("theta.csv") {
            Err(_) => panic!("Can not create theta.csv"),
            Ok(file) =>file,
        };
        match write!(file, "{},{}", val.0, val.1) {
            Err(why) => panic!("Could not write {} because {}", path.display(), Error::description(&why)),
            Ok(_) => println!("Final values : t0 = {}, t1 = {}", val.0, val.1),
        }

        let dur = instant.elapsed();
        println!("training took : {}.{} seconds", dur.as_secs(), dur.subsec_nanos());
    } else {
        println!("use is `learning [file] [iteration]`");
        println!("Default iteration is 5500.");
    }
}
