use std::cmp::Ordering;
use advent_of_code_2022::{get_content, DataType};

fn is_range_contained(ranges: &Vec<usize>) -> bool {
    match ranges[0].cmp(&ranges[2]) {
        Ordering::Less => {
            match ranges[1].cmp(&ranges[3]) {
                Ordering::Greater | Ordering::Equal => true,
                Ordering::Less => false,
            }
        },
        Ordering::Greater => {
            match ranges[1].cmp(&ranges[3]) {
                Ordering::Less | Ordering::Equal => true,
                Ordering::Greater => false,
            }
        },
        Ordering::Equal => true,
    }
}

// No overlap
// |---------|
//              |------------|
// 
//           |----|
//  |-----|
//
// Overlap
// |-----------|
//         |--------------|
//
//           |----------|
//    |---------|
//
fn do_assignments_overlap(ranges: &Vec<usize>) -> bool {
    if ranges[0] > ranges[3] { return false }
    if ranges[2] > ranges[1] { return false }
    true
}
fn numerize(input: &str) -> usize {
    match input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => std::process::exit(0),
    }
}

fn main() {
    let content = get_content(4, DataType::Input);
    let mut nb_of_pair_of_assignments_that_contains_themselves = 0;
    let mut nb_of_assignments_overlapping = 0;
    for line in content.lines() {
        let elves_assignment: Vec<usize> = line.split(&[',', '-'])
            .map(|x| numerize(x))
            .collect();
        // println!("{:?}", elves_assignment);
        // println!("Contained ? : {}", is_range_contained(elves_assignment));
        match is_range_contained(&elves_assignment) {
            true => nb_of_pair_of_assignments_that_contains_themselves += 1,
            _ => (),
        }

        match do_assignments_overlap(&elves_assignment) {
            true => nb_of_assignments_overlapping += 1,
            _ => (),
        }
    }
    println!("========== DAY 04 ===========");
    println!("Part 1 : {}", nb_of_pair_of_assignments_that_contains_themselves);
    println!("Part 2 : {}", nb_of_assignments_overlapping);
}

//        match goal_value.cmp(&player_input) {
//            Ordering::Less => {
//                println!("Lesser .. {player_input} is too high. Continue !")
//            }
//            Ordering::Equal => {
//                println!("Correct .. cheater.");
//                break;
//            }
//            Ordering::Greater => {
//                println!("Greater .. {player_input} is too small. continue.")
//            }
//        }
