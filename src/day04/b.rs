use std::fs::File;
use std::io::prelude::*;

const PATTERNS: [[[char; 3]; 3]; 4] = [
    [
        ['M', '.', 'M'],
        ['.', 'A', '.'],
        ['S', '.', 'S'],
    ],
    [
        ['M', '.', 'S'],
        ['.', 'A', '.'],
        ['M', '.', 'S'],
    ],
    [
        ['S', '.', 'S'],
        ['.', 'A', '.'],
        ['M', '.', 'M'],
    ],
    [
        ['S', '.', 'M'],
        ['.', 'A', '.'],
        ['S', '.', 'M'],
    ],
];


pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day04/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let grid: Vec<Vec<char>> = text.lines().map(|x| x.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut count = 0;

    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let mut found_pattern = false;
            
            'PTRN:
            for pattern in PATTERNS {
                for y2 in 0..3 {
                    for x2 in 0..3 {
                        let ax = y + y2;
                        let ay = x + x2;
                        let pattern_char = pattern[y2 as usize][x2 as usize];

                        if pattern_char != '.' && grid[ay as usize][ax as usize] != pattern_char {
                            continue 'PTRN;
                        }
                    }
                }

                found_pattern = true;
                break;
            }

            if found_pattern {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
