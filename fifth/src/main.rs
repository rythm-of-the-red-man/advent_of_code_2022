use std::fs;
fn main() {
    let mut crates: Vec<Vec<&str>> = vec![
        vec!["N", "T", "B","S",  "Q", "H", "G", "R"],
        vec!["J", "Z", "P", "D", "F", "S", "H"],
        vec!["V", "H", "Z"],
        vec!["H", "G", "F", "J", "Z", "M"],
        vec!["R", "S", "M", "L", "D", "C", "Z", "T"],
        vec!["J", "Z", "H", "V", "W", "T", "M"],
        vec!["Z", "L", "P", "F", "T"],
        vec!["S", "W", "V", "Q"],
        vec!["C", "N", "D", "T", "M", "L", "H", "W"],
    ];
    let file_input = fs::read_to_string("./input.txt")
        .expect("Can't read file")
        .replace("from", ",")
        .replace("to", ",")
        .replace(" ", "")
        .replace("move", "");
    let instructions: Vec<(i32, i32, i32)> = file_input
        .split("\n")
        .map(|a| {
            let result: Vec<&str> = a.split(',').collect();
            (
                result[0].parse::<i32>().unwrap(),
                result[1].parse::<i32>().unwrap()-1,
                result[2].parse::<i32>().unwrap()-1,
            )
        })
        .collect();

    for instruction in instructions {
        let mut extension: Vec<&str> = crates[instruction.1 as usize]
            .drain(0..instruction.0 as usize)
            .collect();
        crates[instruction.2 as usize].reverse();
        crates[instruction.2 as usize].extend(extension);
        crates[instruction.2 as usize].reverse();    }
    for _crate in crates {
        println!("{:?}",_crate[0]);
    }
}
// PTWLTDSJV single crate
// WZMFVGGZP mutliple crates 
