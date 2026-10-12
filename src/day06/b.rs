use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day06/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    
    let grid: Vec<Vec<char>> = text.lines().map(|x| x.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32; 

    let mut start = (0, 0);

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'^' {
                start = (x as i32, y as i32)
            }
        }
    }

    let mut current_direction = 0;
    let mut visited = HashSet::new();

    'outside:
    loop {
        let delta = DIRECTIONS[current_direction];

        loop {
            let next = (start.0 + delta.0, start.1 + delta.1);


            if (0..width).contains(&next.0) && (0..height).contains(&next.1) {
                if grid[next.1 as usize][next.0 as usize] == '#' {
                    current_direction = (current_direction + 1) % 4;
                    break;
                }
            } else {
                break 'outside;
            }

            visited.insert(start);
            start = next;
        }
    }

    println!("{}", visited.len() + 1);

    Ok(())
}
