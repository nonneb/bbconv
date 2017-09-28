use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1:f64 = args[1].parse().unwrap();
    let arg2:f64 = args[2].parse().unwrap();
    let arg3:f64 = args[3].parse().unwrap();
    let arg4:f64 = args[4].parse().unwrap();

    println!("{:?}", args);
    println!("{}, {}, {}, {}", arg1, arg2, arg3, arg4);

}
