use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

fn has_loop(grid: Vec<Vec<char>>, start: (i32, i32), (width, height): (i32, i32)) -> bool {
    let mut current = start;
    let mut current_direction = 0;
    let mut visited = HashSet::new();

    loop {
        let delta = DIRECTIONS[current_direction];

        loop {
            let next = (current.0 + delta.0, current.1 + delta.1);            

            if visited.contains(&(next, current_direction)) {                
                return true;
            }

            if !(0..width).contains(&next.0) || !(0..height).contains(&next.1) {                
                return false;
            }

            visited.insert((current, current_direction));

            if grid[next.1 as usize][next.0 as usize] == '#' {
                current_direction = (current_direction + 1) % 4;
                break;
            } else {
                current = next;
            }
        }
    }
}

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day06/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    
    let mut grid: Vec<Vec<char>> = text.lines().map(|x| x.chars().collect()).collect();
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

    grid[start.1 as usize][start.0 as usize] = '.';
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let mut grid_clone = grid.clone();
            grid_clone[y as usize][x as usize] = '#';

            if has_loop(grid_clone, start, (width, height)) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
