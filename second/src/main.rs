use std::fs;
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum YourPossibilites {
    X,
    Y,
    Z,
}

impl YourPossibilites {
    fn from_string(possibility: &str) -> YourPossibilites {
        if possibility == "X" {
            return YourPossibilites::X
        } else if possibility == "Y" {
            return YourPossibilites::Y
        } else if possibility == "Z" {
            return YourPossibilites::Z
        } else {
            panic!("Unknown possibility for your choice")
        }
    }
    fn to_rps(&self)->RPS{
        match &self {
            YourPossibilites::X => RPS::Rock,
            YourPossibilites::Y => RPS::Paper,
            YourPossibilites::Z => RPS::Scissors,
        }
    }
    fn from_string_based_on_oponent_choice(possibility: &str, opponent_choice:&RPS)->RPS{
        let possibility = YourPossibilites::from_string(possibility);
        match opponent_choice{
            RPS::Rock => {
                match possibility {
                    YourPossibilites::X => RPS::Scissors,
                    YourPossibilites::Y => RPS::Rock,
                    YourPossibilites::Z => RPS::Paper
                }
            },
            RPS::Paper => {
                match possibility {
                    YourPossibilites::X => RPS::Rock,
                    YourPossibilites::Y => RPS::Paper,
                    YourPossibilites::Z => RPS::Scissors
                }
            },
            RPS::Scissors => {
                match possibility {
                    YourPossibilites::X => RPS::Paper,
                    YourPossibilites::Y => RPS::Scissors,
                    YourPossibilites::Z => RPS::Rock
                }
            },
        }
    }
}
enum OpponentPosibilites {
    A,
    B,
    C,
}
impl OpponentPosibilites {
    fn from_string(possibility: &str) -> OpponentPosibilites {
        if possibility == "A" {
            return OpponentPosibilites::A;
        } else if possibility == "B" {
            return OpponentPosibilites::B;
        } else if possibility == "C" {
            return OpponentPosibilites::C;
        } else {
            panic!("Unknown possibility for your choice")
        }
    }
    fn to_rps(&self) -> RPS{
        match &self {
            OpponentPosibilites::A => RPS::Rock,
            OpponentPosibilites::B => RPS::Paper,
            OpponentPosibilites::C => RPS::Scissors,
        }
    }
}
struct RPSMatch {
    your_choice: RPS,
    opponent_choice: RPS,
}

impl RPSMatch {
    fn get_score(&self) -> i32 {
        RPSMatch::get_final_result(&self.your_choice, &self.opponent_choice)
    }
    fn get_final_result(choice: &RPS, opponent_choice: &RPS) -> i32 {
        RPSMatch::get_base_score(choice) + RPSMatch::get_points_for_result(choice, opponent_choice)
    }
    fn get_base_score(choice: &RPS) -> i32 {
        match choice {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn get_points_for_result(choice: &RPS, opponent_choice: &RPS) -> i32 {
        match choice {
            RPS::Paper => match opponent_choice {
                RPS::Rock => 6,
                RPS::Paper => 3,
                RPS::Scissors => 0,
            },
            RPS::Rock => match opponent_choice {
                RPS::Rock => 3,
                RPS::Paper => 0,
                RPS::Scissors => 6,
            },
            RPS::Scissors => match opponent_choice {
                RPS::Rock => 0,
                RPS::Paper => 6,
                RPS::Scissors => 3,
            },
        }
    }
}
fn main() {
    let file_input = fs::read_to_string("./input.txt").expect("Can't read file");
    let matches: Vec<i32> = file_input
        .split("\n")
        .map(|item| {
            let raw_choices: Vec<&str> = item.split(' ').collect();
            let opponent_choice = OpponentPosibilites::from_string(raw_choices[0]).to_rps();
            let rps_match = RPSMatch{
                your_choice:YourPossibilites::from_string_based_on_oponent_choice(raw_choices[1], &opponent_choice),
                opponent_choice
            };
            rps_match.get_score()
        })
        .collect();
    println!("{:?}",matches.iter().sum::<i32>())
}
