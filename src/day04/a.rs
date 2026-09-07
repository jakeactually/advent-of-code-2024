use std::fs::File;
use std::io::prelude::*;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_in_grid_with_value(grid: &Vec<Vec<char>>, bounds: (i32, i32), position: (i32, i32), value: char) -> bool {
    bounds.0 > position.0
        && position.0 >= 0
        && bounds.1 > position.1
        && position.1 >= 0
        && grid[position.1 as usize][position.0 as usize] == value
}

pub fn look_for_word(grid: &Vec<Vec<char>>, bounds: (i32, i32), start: (i32, i32)) -> i32  {        
    let mut amount = 0;

    for dir in DIRECTIONS.iter() {
        let (mut cx, mut cy) = (start.0, start.1);
        let mut valid = true;

        for letter in "XMAS".chars() {
            if !is_in_grid_with_value(grid, bounds, (cx, cy), letter) {
                valid = false;
                break;
            }

            (cx, cy) = (cx + dir.0, cy + dir.1);
        }

        if valid {
            amount += 1;
        }
    }

    return amount;
}

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day04/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let grid: Vec<Vec<char>> = text.lines().map(|x| x.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            count += look_for_word(&grid, (width, height), (x, y));
        }
    }

    println!("{}", count);

    Ok(())
}
