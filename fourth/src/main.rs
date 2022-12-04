use std::fs;
fn main() {
    let file_input = fs::read_to_string("./input.txt").expect("Can't read file");
    let pairs: Vec<&str> = file_input.split("\n").collect();
    let numbers = pairs.into_iter().map(|pair| {
        let first_split: Vec<&str> = pair.split(',').collect();
        let first_elve: Vec<i32> = first_split[0]
            .split("-")
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        let second_elve: Vec<i32> = first_split[1]
            .split("-")
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        (first_elve[0], first_elve[1], second_elve[0], second_elve[1])
    });
    let mut result: Vec<(i32, i32, i32, i32)> = vec![];
    for num in numbers {
        result.push(num);
    }
    let mut sum = 0;
    for num in result {
        if num.0 >= num.2 && num.1 <= num.3 // 1st star
            || num.2 >= num.0 && num.3 <= num.1 //1st star

            || num.1 >= num.2 && num.1 <= num.3 //2nd star end of first range happens in second range
            || num.0 >= num.2 && num.0 <= num.3 // 2nd starstart of first range happend in second rage

            || num.3 >= num.0 && num.3 <= num.1//2nd star
            || num.2 >= num.0 && num.2 <= num.1//2nd star
        {
            sum += 1
        }
    }
    println!("{:?}", sum);
}
