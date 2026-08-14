use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day03/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut enabled = true;

    for captures in re.captures_iter(&text) {
        if captures.get_match().as_str() == "do()" {
            enabled = true;
        } else if captures.get_match().as_str() == "don't()" {
            enabled = false;
        } else {
            let a= captures.get(1).unwrap().as_str();
            let b= captures.get(2).unwrap().as_str();
            if enabled {
                total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", total);

    Ok(())
}