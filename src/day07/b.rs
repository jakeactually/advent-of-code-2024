use std::fs::File;
use std::io::prelude::*;

fn drop_two_add_one(vec: &Vec<u64>, value: u64) -> Vec<u64> {
    let mut new_vec = vec[0..vec.len() - 2].to_vec();
    new_vec.push(value);
    return new_vec;
}

fn nearest_power_of_ten(n: u64) -> u64 {
    let mut result = 10;

    loop {
        if result > n {
            return result;
        }

        result *= 10;
    }
}

fn backtrack(acc: Vec<u64>) -> bool {
    if acc.len() == 2 {
        return acc[0] == acc[1];
    }

    let last = acc[acc.len() - 1];
    let second_to_last = acc[acc.len() - 2];

    if last < second_to_last {
        return false;
    }

    let difference = last - second_to_last;
    let addition = backtrack(drop_two_add_one(&acc, difference));

    let multiplication_vec = drop_two_add_one(&acc, last / second_to_last);
    let multiplication = last % second_to_last == 0 && backtrack(multiplication_vec);

    let power_of_ten = nearest_power_of_ten(second_to_last);
    let concatenation_vec = drop_two_add_one(&acc, difference / power_of_ten);
    let concatenation = difference % power_of_ten == 0 && backtrack(concatenation_vec);

    return addition || multiplication || concatenation;
}

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day07/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let mut total = 0;

    for line in text.lines() {
        let (result_txt, parts_txt) = line.split_once(": ").unwrap();
        let result = result_txt.parse::<u64>().unwrap();
        let mut parts = parts_txt.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        parts.push(result);

        if backtrack(parts) {
            total += result;
        }
    }

    println!("{}", total);

    Ok(())
}
