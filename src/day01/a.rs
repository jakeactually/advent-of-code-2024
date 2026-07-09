use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day01/input.txt").map_err(|e| e.to_string())?;

    let mut s = String::new();
    file.read_to_string(&mut s).map_err(|e| e.to_string())?;

    println!("{}", s);

    Ok(())
}
