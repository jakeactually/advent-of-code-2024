use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day07/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    for line in text.lines() {
        let (result_txt, parts_txt) = line.split_once(": ").unwrap();
        let result = result_txt.parse::<u32>().unwrap();
        let parts = parts_txt.split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        dbg!((result, parts));
    }

    Ok(())
}