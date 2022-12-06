use std::fs;
fn main() {
    let file_input = fs::read_to_string("./input.txt").expect("Can't read file");
    let mut counter = 0;
    let mut last_four: Vec<char> = vec![];
    for ch in file_input.chars() {
        counter += 1;

        if last_four.contains(&ch) {
            let same_char_index = last_four.iter().position(|&x| x == ch).unwrap();
            last_four.drain(0..same_char_index+1);
        }
            last_four.push(ch);
        if last_four.len() == 14 {
            println!("{:?}", counter);
            break;
        }
    }
}
