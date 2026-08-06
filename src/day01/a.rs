use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day01/input.txt").map_err(|e| e.to_string())?;

    let mut s = String::new();
    file.read_to_string(&mut s).map_err(|e| e.to_string())?;

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in s.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left_num: i32 = left.parse().unwrap();
        let right_num: i32 = right.parse().unwrap();
        left_vec.push(left_num);
        right_vec.push(right_num);
    }

    left_vec.sort();
    right_vec.sort();
    let mut total = 0;

    for (left_num, right_num) in left_vec.iter().zip(right_vec) {
        total += (left_num - right_num).abs();
    }

    println!("Total: {}", total);

    Ok(())
}
