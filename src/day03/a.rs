use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day03/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    Ok(())
}