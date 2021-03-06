#![feature(slice_patterns)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 5 {
        println!("Please enter text in the format '50 gallons to cups.'");
    } else {
    let arg1:f64 = args[1].parse().unwrap();
    let arg2: &str = &args[2];
    let arg3: &str = &args[3];
    let arg4: &str = &args[4];

    if arg2 == "pounds" && arg4 == "cups" {
        println!("{} {} is {} {}", arg1, &arg2, poundstocups(arg1), &arg4);
    } else if arg2 == "cups" && arg4 == "pounds" {
        println!("{} {} is {} {}", arg1, &arg2, cupstopounds(arg1), &arg4);
    } else if arg2 == "quarts" && arg4 == "pounds" {
        println!("{} {} is {} {}", arg1, &arg2, quartstopounds(arg1), &arg4);
    } else if arg2 == "pounds" && arg4 == "quarts" {
        println!("{} {} is {} {}", arg1, &arg2, poundstoquarts(arg1), &arg4);
    
    } else {
        println!("Not implemented");
    }
    }
    
    

}

fn poundstocups(x: f64) -> f64 {
    let y = 3 as f64;
    x * y
}

fn cupstopounds(x: f64) -> f64 {
    let y = 3 as f64;
    x / y 
}

fn quartstopounds(x: f64) -> f64 {
   x * 1.5
}

fn poundstoquarts(x: f64) -> f64 {
    x / 1.5
}
