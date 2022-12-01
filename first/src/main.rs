use std::env;
use std::fs;
use std::slice::Split;

fn main() {
    let file_input = fs::read_to_string("./input.txt").expect("Can't read file");
    let all_elves: Vec<i32> = file_input
        .split("\n")
        .map(|item| {
            let result = item.parse::<i32>();
            match result {
                Ok(result) => result,
                Err(_) => -1,
            }
        })
        .collect();

    let mut all_elves_splitted = vec![vec![]];
    let mut current_elve = 0;

    for elve in all_elves {
        if elve == -1 {
            all_elves_splitted.push(vec![]);
            current_elve += 1;
            continue;
        }
        all_elves_splitted[current_elve].push(elve)
    }
    let mut summed: Vec<i32> = all_elves_splitted
        .iter()
        .map(|one_elve| one_elve.iter().sum::<i32>()).collect();
    summed.sort();
    println!("{:?}", summed.pop().unwrap() +summed.pop().unwrap() +summed.pop().unwrap() );
}