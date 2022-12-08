use std::fs;

fn main() {
    let file_input = fs::read_to_string("./input.txt").unwrap();
    let mut vector_of_dirts: Vec<(String, i64)> = vec![];
    let mut current_dir: Vec<String> = vec![];

    let mut dirs_vector: Vec<String> = vec![];

    for line in file_input.split("\n") {
        // if line.contains('/'){

        // }
        if line.contains("$ cd ") {
            let dir = line.replace("$ cd ", "");
            if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir.to_string());
                vector_of_dirts.push((current_dir.join("/"), 0));
            }
        } else if line.contains("dir ") {
        } else if line.contains("$ ls") {
            // println!("{}",line);
        } else {
            // println!("{:?}", current_dir);
            let parsed: Vec<&str> = line.split(" ").collect();
            let size2 = parsed[0].parse::<i64>().unwrap();
            for i in 0..current_dir.len()+1 {
                let key = current_dir[0..i].join("/");
                let to_append_index = vector_of_dirts.iter().position(|item| &item.0 == &key);
                match to_append_index {
                    Some(item) => {
                        vector_of_dirts[item].1 += size2;
                    }
                    None => println!("did not found {}",key),
                }
            }
        }
    }
    println!("{:?}",vector_of_dirts);
    let result = vector_of_dirts
        .iter()
        .filter(|item| item.1 <= 100000)
        .map(|item| item.1)
        .sum::<i64>();

    vector_of_dirts.sort_by(|a,b| b.1.cmp(&a.1));
    let to_free = 30000000 - (70000000-vector_of_dirts[0].1);

    for dir in vector_of_dirts{
        if dir.1 > to_free{
            println!("{:?}",dir);
        }else{
            break
        }
    }
    
    // 977520
    // 977520
    // 175581
    // 175581
    // 1350966
}
