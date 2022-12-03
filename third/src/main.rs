use std::fs;
fn main() {
    fn char_to_priority(ch: char) -> u32 {
        let as_decimal = ch as u32;
        if as_decimal < 97 {
            return as_decimal - 65 + 1 + 26;
        }
        return as_decimal - 97 + 1;
    }
    let file_input = fs::read_to_string("./input.txt").expect("Can't read file");
    let elves: Vec<u32> = file_input.split("\n").map(|item|{
        let halved = item.split_at(item.len()/2);
        for ch in halved.0.chars(){

            if halved.1.contains(ch){
                return char_to_priority(ch)
            }
        }
        0
    } ).collect();
    println!("{:?}",elves.iter().sum::<u32>());

    // Second star
    let file_input2 = fs::read_to_string("./input.txt").expect("Can't read file");
    let elves = file_input2.split("\n");
    let mut elves_vec: Vec<&str> = vec![];
    for e in elves{
        elves_vec.push(e);
    }
    let mut i = 0;
    let mut sum = 0;
    while i < elves_vec.len(){
        let first_elve = elves_vec[i];
        let second_elve = elves_vec[i+1];
        let third_elve = elves_vec[i+2];
        for ch in first_elve.chars() {
            if second_elve.contains(ch) && third_elve.contains(ch){
                sum+= char_to_priority(ch);
                i+=3;
                break;
            }
            
        }
    }
    println!("{:?}",sum);
}
