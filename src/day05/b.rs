use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn bubble_sort(pages: &mut [&str], tuples: &HashSet<(&str, &str)>) {
    for end in (1..pages.len()).rev() {
        let mut swapped = false;

        for i in 0..end {
            if tuples.contains(&(pages[i + 1], pages[i])) {
                pages.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day05/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    let (tuples_str, pages_str) = text.split_once("\n\n").unwrap();

    let tuples = tuples_str
        .lines()
        .map(|x| x.split_once("|").unwrap())
        .collect::<HashSet<_>>();

    let mut total = 0;

    for page_str in pages_str.lines() {
        let mut pages = page_str.split(",").collect::<Vec<_>>();
        let mut valid = true;

        for i in 0..pages.len() {
            for j in (i + 1)..pages.len() {
                if !tuples.contains(&(pages[i], pages[j])) {
                    valid = false;
                }
            }
        }

        if !valid {
            bubble_sort(&mut pages, &tuples);
            total += pages[pages.len() / 2].parse::<i32>().unwrap();
        }
    }

    println!("{}", total);

    Ok(())
}
