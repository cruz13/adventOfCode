use advent_of_code_2022::{get_content, DataType};

//const range1:String = (b'a' .. b'z').map(char::from);

fn extract_compartments(rucksack: &str) -> Vec<String> {
    let string_length = rucksack.chars().count();
    let first_compartment = &rucksack[0..string_length/2];
    let second_compartment = &rucksack[string_length/2..];
    //println!("{} : {} - {}", rucksack, first_compartment, second_compartment);
    vec!(first_compartment.to_string(), second_compartment.to_string())
}

fn compare_compartments(compartment_1: &String, compartment_2: &String) -> Vec<char> {
    let mut chars_in_both_compartments = Vec::new();
    for char in compartment_1.chars() {
        for char_in_second_compartment in compartment_2.chars() {
           if char == char_in_second_compartment && ! chars_in_both_compartments.contains(&char) {
               chars_in_both_compartments.push(char);
           }
        }
    }
    chars_in_both_compartments 
}

fn compare_rucksacks(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> Vec<char> {
    // Should probably use one function for `compare_compartments` and `compare_rucksacks`
    let mut chars_in_common = Vec::new();
    for char in rucksack1.chars() {
        for char2 in rucksack2.chars() {
            for char3 in rucksack3.chars() {
                if char == char2 && char == char3 && ! chars_in_common.contains(&char) {
                    chars_in_common.push(char);
                }
            }
        }
    }
    chars_in_common
}

fn calculate_sum_of_priorities(items: Vec<char>) -> usize {
    let mut range1:Vec<char> = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    let mut range2:Vec<char> = (b'A'..=b'Z').map(char::from).collect::<Vec<_>>();
    range1.append(&mut range2);
    let mut sum_of_priorities = 0;
    //println!("{:?}", range1);
    for item in items {
        let priority: usize = range1.iter().position(|&c| c == item).unwrap() + 1;
        sum_of_priorities += priority;
    }
    sum_of_priorities
}

fn main() {
    let content = get_content(3, DataType::Input);

    let mut common_items = Vec::new();
    let mut elf = 0;
    let mut group_of_elves: Vec<&str> = Vec::new();
    let mut badges = Vec::new();

    for line in content.lines() {
        let mut common_item = compare_compartments(&extract_compartments(line)[0], &extract_compartments(line)[1]); 
        common_items.append(&mut common_item);

        elf += 1;
        group_of_elves.push(line);
        println!("Pushing line {} into group : {:?}", line, group_of_elves);
        if elf % 3 == 0 {
            println!("Group of elves has the following 3 rucksacks: {:?}", group_of_elves);
            let mut badge = compare_rucksacks(group_of_elves[0], group_of_elves[1], group_of_elves[2]);
            badges.append(&mut badge);
            group_of_elves.clear();
        }
    }
    // println!("{:?}", common_items);
    println!("Part 1 : Sum of priorities: {}", calculate_sum_of_priorities(common_items));
    println!("Part 2 : badges: {}", calculate_sum_of_priorities(badges));
}
