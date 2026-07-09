use std::env;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let part = args[2].as_str();

    match (day, part) {
        ("01", "a") => day01::a::run(),
        _ => panic!("Unknown day or part"),
    }
}
