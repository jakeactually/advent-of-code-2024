use std::env;

mod day01;
mod day02;
mod day03;
mod scaffold;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let part = args[2].as_str();

    let res = match (day, part) {
        ("day", other) => scaffold::scaffold_day(other),
        ("01", "a") => day01::a::run(),
        ("01", "b") => day01::b::run(),
        ("02", "a") => day02::a::run(),
        ("02", "b") => day02::b::run(),
        ("03", "a") => day03::a::run(),
        ("03", "b") => day03::b::run(),
        _ => Err("Unknown day or part".to_string()),
    };

    if let Err(e) = res {
        println!("{}", e);
    }
}
