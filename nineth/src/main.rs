use std::fs;
fn main() {
    let binding = fs::read_to_string("./input.txt").expect("Can't read file");

    let instructions: Vec<(&str, u32)> = binding
        .split("\n")
        .map(|item| {
            let splitted: Vec<&str> = item.split(' ').collect();
            return (splitted[0], splitted[1].parse::<u32>().unwrap());
        })
        .collect();
    let mut tail_places: Vec<(i32, i32)> = vec![(0, 0)];
    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    for instruction in instructions {
        for i in 0..instruction.1 {
            let old_head_position = move_head(instruction.0, &mut head_position);
            move_tail(
                &head_position,
                old_head_position,
                &mut tail_position,
                &mut tail_places,
            );
        }
    }
    fn move_head(direction: &str, head_position: &mut (i32, i32)) -> (i32, i32) {
        let old_position = head_position.clone();
        if direction == "U" {
            head_position.0 += 1;
        } else if direction == "D" {
            head_position.0 -= 1;
        } else if direction == "L" {
            head_position.1 -= 1;
        } else if direction == "R" {
            head_position.1 += 1;
        }
        old_position
    }
    fn move_tail(
        head_position: &(i32, i32),
        old_head_position: (i32, i32),
        tail_position: &mut (i32, i32),
        tail_places: &mut Vec<(i32, i32)>,
    ) {
        if (head_position.0 - tail_position.0).abs() > 1
            || (head_position.1 - tail_position.1).abs() > 1
        {
            tail_position.0 = old_head_position.0;
            tail_position.1 = old_head_position.1;
            tail_places.push(*tail_position);
        }
    }
    tail_places.sort();
    tail_places.dedup();

    println!("{:?}", tail_places.len());

    let instructions2: Vec<(&str, u32)> = binding
        .split("\n")
        .map(|item| {
            let splitted: Vec<&str> = item.split(' ').collect();
            return (splitted[0], splitted[1].parse::<u32>().unwrap());
        })
        .collect();
    let mut knots: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut tail_places2: Vec<(i32, i32)> = vec![(0, 0)];
        // let mut counter = 1 ;
    let knots_len = knots.len();
    let mut last_knot_position = (0,0);
    for instruction in instructions2 {
        for i in 0..instruction.1 {
            last_knot_position = move_head(instruction.0, &mut knots[0]);
            for i in 1..knots.len() {
                println!("{:?}",knots);
                last_knot_position = move_knot(
                    knots[i-1],
                    last_knot_position,
                    &mut knots[i],
                    &mut tail_places2,
                    i == knots_len-1,
                );
            }
        }
    }
    // println!("{:?}", tail_places2.len());

    tail_places2.sort();
    tail_places2.dedup();

    println!("{:?}", tail_places2.len());
    fn move_knot(
        head_position: (i32, i32),
        old_head_position: (i32, i32),
        knot_position: &mut (i32, i32),
        tail_places: &mut Vec<(i32, i32)>,
        should_push: bool,
    ) ->(i32,i32){
        let old_position = knot_position.clone();
        let not_touching = (head_position.0 - knot_position.0).abs() > 1 ||  (head_position.1 - knot_position.1).abs() > 1;
        let in_same_row = head_position.0 == knot_position.0 || head_position.1 == knot_position.1;
        if not_touching && !in_same_row{
            if head_position.0 >= knot_position.0{
                knot_position.0+=1;
            }else{
                knot_position.0-=1;
            }
            if head_position.1 >= knot_position.1{
                knot_position.1+=1;
            }else{
                knot_position.1-=1;
            }
            if should_push{
                tail_places.push(*knot_position);
            }
            return old_position
        }
        else if in_same_row && not_touching {
            if knot_position.0 == head_position.0 {
                if head_position.1 > knot_position.1{
                    knot_position.1 +=1
                }else{
                    knot_position.1 -=1
                }
            }else{
                if head_position.0 > knot_position.0{
                    knot_position.0 +=1
                }else{
                    knot_position.0 -=1
                }
            }

            if should_push{
                tail_places.push(*knot_position);
            }
            return old_position

        }
        // else if not_touching
        // {
        //     knot_position.0 = old_head_position.0;
        //     knot_position.1 = old_head_position.1;
        //     if should_push{
        //         tail_places.push(*knot_position);
        //     }
        //     return old_position
        // }
        else{
            if should_push{
                tail_places.push(*knot_position);
            }
            return old_position
        }
    }
}

// 11 23

// h z 14 na 24 // 1 z 03 na 14 // 2 z 02 na 13
