use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day02/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let mut valid = 0;

    for line in text.lines() {
        let digits: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let mut diffs = Vec::new();

        for i in 0..digits.len() - 1 {
            let diff = digits[i] - digits[i + 1];
            diffs.push(diff);
        }

        let in_limits = diffs.iter().all(|x| (1..4).contains(&x.abs()));
        let descending = diffs.iter().all(|x| *x > 0);
        let ascending = diffs.iter().all(|x| *x < 0);

        if in_limits && (descending || ascending) {
            valid += 1;
        }
    }

    println!("{}", valid);

    Ok(())
}