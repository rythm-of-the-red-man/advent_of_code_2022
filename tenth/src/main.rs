use std::fs;

fn main() {
    let binding = fs::read_to_string("./input.txt").expect("Can't read file");
    fn noop(cycle: &mut i32, X: &mut i32, letter: &mut Vec<&str>) {
        increment_cycle(cycle, X, letter);
    }
    fn addx(val: i32, cycle: &mut i32, X: &mut i32, letter: &mut Vec<&str>) {
        noop(cycle, X, letter);
        increment_cycle(cycle, X, letter);
        *X += val;
    }
    fn inspector(cycle: &mut i32, X: &mut i32) {
        let inspector_stops = [20, 60, 100, 140, 180, 220];
        if inspector_stops.contains(cycle) {
            println!("{}, {}, {}", X, cycle, *X * *cycle);
        }
    }
    fn increment_cycle(cycle: &mut i32, X: &mut i32, letter: &mut Vec<&str>) {
        render(*cycle, *X, letter);
        *cycle += 1;
        inspector(cycle, X);
    }

    fn render(cycle: i32, X: i32, letter: &mut Vec<&str>) {
        if cycle % 40 == 0 {
            letter.push("\n");
        }
        let start = X-1;
        let end = X-1 + 2;
        if start <= cycle%40 && cycle%40 <= end {
            return letter.push("#")
        }
        return letter.push(".")
    }
    let instructions: Vec<&str> = binding.split("\n").collect();
    let mut letter: Vec<&str> = vec![];
    let mut X = 1;
    let mut cycle = 0;
    for instruction in instructions {
        let mut splitted = instruction.split(" ");
        let command = splitted.next().unwrap();

        if command == "addx" {
            let val = splitted.next().unwrap().parse::<i32>().unwrap();
            addx(val, &mut cycle, &mut X, &mut letter);
        } else {
            noop(&mut cycle, &mut X,  &mut letter);
        }
    }
    println!("{}",letter.join(""));
}

//13140
// 15020
