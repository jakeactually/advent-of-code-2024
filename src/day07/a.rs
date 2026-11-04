use std::fs::File;
use std::io::prelude::*;

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
    let mut addition_vec = acc[0..acc.len() - 2].to_vec();
    addition_vec.push(difference);
    let addition = backtrack(addition_vec);

    if last % second_to_last != 0 {
        return addition;
    } else {
        let dividend = last / second_to_last;
        let mut multiplication_vec = acc[0..acc.len() - 2].to_vec();
        multiplication_vec.push(dividend);
        let multiplication = backtrack(multiplication_vec);
        return addition || multiplication;
    }
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
