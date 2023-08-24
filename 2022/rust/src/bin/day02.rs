use std::fs;

const FILE_PATH :&str  = "asset/day02.input.txt";

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
// Part02: X for Lost, Y for Draw, Z for WIN
// (1 for Rock, 2 for Paper, and 3 for Scissors)
// (0 if you lost, 3 if the round was a draw, and 6 if you won)
enum Figure{
    Rock,
    Paper,
    Scissors,
}
enum RoundStatus{
    Win,
    Draw,
    Lost,
}
fn get_figure_point(figure:Figure) -> usize {
    match figure{
        Figure::Rock => 1,
        Figure::Paper => 2,
        Figure::Scissors => 3,
    }
}
fn get_round_status_point(status: RoundStatus) -> usize {
    match status {
        RoundStatus::Win => 6,
        RoundStatus::Draw => 3,
        RoundStatus::Lost => 0,
    }
}

fn get_figure(character: &str) -> Option<Figure> {
    match character{
        "A" | "X" => Some(Figure::Rock),
        "B" | "Y" => Some(Figure::Paper),
        "C" | "Z" => Some(Figure::Scissors),
        _ => None,
    }
}
fn get_strategic_action(character: &str) -> Option<RoundStatus> {
    match character {
        "X" => Some(RoundStatus::Lost),
        "Y" => Some(RoundStatus::Draw),
        "Z" => Some(RoundStatus::Win),
        _ => None,
    }
}
fn get_round_status(opponent_figure: &Figure, player_figure: &Figure) -> RoundStatus {
    match (opponent_figure, player_figure) {
        (Figure::Rock, Figure::Paper) => RoundStatus::Win,
        (Figure::Rock, Figure::Scissors) => RoundStatus::Lost,
        (Figure::Paper, Figure::Rock) => RoundStatus::Lost,
        (Figure::Paper, Figure::Scissors) => RoundStatus::Win,
        (Figure::Scissors, Figure::Rock) => RoundStatus::Win,
        (Figure::Scissors, Figure::Paper) => RoundStatus::Lost,
        _ => RoundStatus::Draw,
    }
}
fn get_strategic_figure(opponent_figure: Figure, strategic_action: &RoundStatus) -> Figure {
    match (opponent_figure, strategic_action) {
        (whatever, RoundStatus::Draw) => whatever,
        (Figure::Rock, RoundStatus::Win) => Figure::Paper,
        (Figure::Rock, RoundStatus::Lost) => Figure::Scissors,
        (Figure::Paper, RoundStatus::Win) => Figure::Scissors,
        (Figure::Paper, RoundStatus::Lost) => Figure::Rock,
        (Figure::Scissors, RoundStatus::Win) => Figure::Rock,
        (Figure::Scissors, RoundStatus::Lost) => Figure::Paper,
    }
}
fn get_round_points(opponent_char: &str, player_char: &str) -> usize {
    let opponent_figure = get_figure(opponent_char).unwrap();
    let player_figure = get_figure(player_char).unwrap();

    let round_status = get_round_status(&opponent_figure, &player_figure);

    get_figure_point(player_figure) + get_round_status_point(round_status)    
}
fn get_strategic_round_points(opponent_char: &str, expected_action: &str) -> usize {
    let opponent_figure = get_figure(opponent_char).unwrap();
    let round_status = get_strategic_action(expected_action).unwrap();
    let player_figure = get_strategic_figure(opponent_figure, &round_status);
    
    get_figure_point(player_figure) + get_round_status_point(round_status)
}


fn main() {

    println!(" ===== DAY 02 ===== ");
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read a file");

    let mut total_score = 0;
    let mut part02_total_score = 0;
    for line in contents.lines() {
        //println!("{}", line);
        let round: Vec<&str> = line.split(" ").collect();
        let round_score = get_round_points(round[0], round[1]);
        total_score += round_score;

        let part02_round_score = get_strategic_round_points(round[0], round[1]);
        part02_total_score += part02_round_score;
    }

    println!("Part01: Total score according to the stragegy guide is : {}", total_score);
    // Total score according to the stragegy guide is : 10816
    println!("Part02: Total score according to the strategy guide is now : {}", part02_total_score); 
}

