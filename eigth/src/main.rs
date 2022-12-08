use std::fs;

fn transpose(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<i32>>())
        .collect()
}
fn main() {
    let heights: Vec<Vec<i32>> = fs::read_to_string("./input.txt")
        .expect("Can't read file")
        .split("\n")
        .map(|item| {
            item.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let mut visible_from_right: Vec<(i32, i32)> = vec![];
    let mut visible_from_left: Vec<(i32, i32)> = vec![];
    let mut i = 0;
    for mut col in heights {
        let mut highest: i32 = -1;
        for tree in 0..col.len() {
            if col[tree] > highest {
                highest = col[tree];
                visible_from_right.push((i as i32, tree as i32))
            }
        }
        col.reverse();
        highest = -1;
        for tree in 0..col.len() {
            if col[tree] > highest {
                highest = col[tree];
                visible_from_left.push((i as i32, (col.len() - 1 - tree) as i32))
            }
        }
        i += 1;
    }
    let heights2: Vec<Vec<i32>> = fs::read_to_string("./input.txt")
        .expect("Can't read file")
        .split("\n")
        .map(|item| {
            item.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let t_heights = transpose(heights2);

    let mut visible_from_top: Vec<(i32, i32)> = vec![];
    let mut visible_from_bottom: Vec<(i32, i32)> = vec![];
    let mut j = 0;

    for mut col in t_heights {
        let mut highest = -1;
        for tree in 0..col.len() {
            if col[tree] > highest {
                highest = col[tree];
                visible_from_top.push((tree as i32, j as i32))
            }
        }
        col.reverse();
        highest = -1;
        for tree in 0..col.len() {
            if col[tree] > highest {
                highest = col[tree];
                visible_from_bottom.push(((col.len() - 1 - tree) as i32, j as i32))
            }
        }
        j += 1;
    }

    let mut concantenated = vec![];

    concantenated.extend(visible_from_top);
    concantenated.extend(visible_from_left);
    concantenated.extend(visible_from_right);
    concantenated.extend(visible_from_bottom);
    concantenated.sort();
    concantenated.dedup();
    println!("{:?}", concantenated.len());

    let heights_3: Vec<Vec<i32>> = fs::read_to_string("./input.txt")
        .expect("Can't read file")
        .split("\n")
        .map(|item| {
            item.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    fn calculate_scenic_score(a: i32, b: i32, map: &Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;
        let map_length = (map[b as usize].len() as i32);
        for i in 0..a {
            if map[i as usize][b as usize] < map[a as usize][b as usize] {
                top += 1;
            } else {
                if i == a{
                    if top == 0 {
                        top = 1;
                        break;
                    }else{
                        break
                    }
                }
                top = 1;
            }
        }
        for i in a+1..map_length {
            if map[i as usize][b as usize] < map[a as usize][b as usize] {
                bottom += 1;
            } else {
                bottom += 1;
                break;
            }
        }
        let my_map = map.clone();

        let trasposed_map = transpose(my_map);
        for i in b+1..map_length {

            if trasposed_map[i as usize][a as usize] < trasposed_map[b as usize][a as usize] {
                right += 1;
            } else {
                right += 1;
                break;
            }
        }
        for i in 0..b {
            if trasposed_map[i as usize][a as usize] < trasposed_map[b as usize][a as usize] {
                left += 1;
            } else {
                if i == b{
                    if left == 0 {
                        left = 1;
                        break;
                    }else{
                        break
                    }
                }
                left = 1;
            }
        }
        return top * bottom * left * right;
    }
    let mut my_map = vec![];
    for i in 0..heights_3.len() {
        let mut row = vec![];
        for j in 0..heights_3[i].len() {
            let test = calculate_scenic_score(i as i32, j as i32, &heights_3);
            row.push(test);
        }
        my_map.push(row);
    }
    let mut max = 0;
    for i in 0..my_map.len(){
        for j in 0..my_map[i].len(){
            if my_map[i][j] > max {
                max = my_map[i][j];
            }
        }
    }
    println!("{:?}",max)
}

