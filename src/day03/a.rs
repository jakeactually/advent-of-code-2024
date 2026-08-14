use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day03/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;


    for (_, [a, b]) in re.captures_iter(&text).map(|c| c.extract()) {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    }

    println!("{}", total);

    Ok(())
}