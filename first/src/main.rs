use std::fs;

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
    let all_elves_split = all_elves.split(|separator| {
        let foo = *separator;
        foo == -1
    });

    let mut all_elves_splitted = vec![];
    for slice in all_elves_split {
        all_elves_splitted.push(slice)
    }
    let mut summed: Vec<i32> = all_elves_splitted
        .iter()
        .map(|one_elve| one_elve.iter().sum::<i32>())
        .collect();

    // 2nd star
    summed.sort();
    let mut _sum = 0;
    for _ in 0..3 {
        _sum += summed.pop().unwrap();
    }
    println!(
        "{:?}",_sum
    );
}
