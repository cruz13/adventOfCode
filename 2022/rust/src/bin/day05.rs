use advent_of_code_2022::{get_content, DataType};

fn build_stacks(stacks_definition: Vec<&str>) -> Vec<Vec<char>> {
    // for stacks in stacks_definition {
    //     println!("{}", stacks);
    // }

    // retrieve the number of stacks we will need:
    // read last digits of the last entry
    let stack_ids = stacks_definition.last().unwrap().trim().split(' ').collect::<Vec<&str>>();
    let nb_of_stacks = stack_ids.last().unwrap().parse::<usize>().unwrap();
    println!("stack id list: {:?} and .. we need : {} crates", stack_ids, nb_of_stacks);

    // read the entries in reverse, and add the crates to each stack
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..nb_of_stacks {
        stacks.push(Vec::new());
    }

    // read `stacks_definition` in reverse, skip the first element (because it contains the crates
    // numbers
    for line in stacks_definition.iter().rev().skip(1) {
         // println!("{:?}", line.char_indices().filter(|(x, _)| x % 4 == 1 ).map(|(_, y)| y).collect::<Vec<char>>());
         let crates_level = line.char_indices().filter(|(x, _)| x % 4 == 1 ).map(|(_, y)| y).collect::<Vec<char>>();
         for (stack_index, crt) in crates_level.iter().enumerate() {
             if *crt != ' ' {
                stacks[stack_index].push(*crt);
             }
         }
    };
    println!("{:?}", stacks_definition);
    stacks
}


fn main() {
    let content = get_content(5, DataType::Exemple);

    let empty_line_index = content.lines().position(|x| x.is_empty()).unwrap();

    let stacks_definition = content.lines().take(empty_line_index).collect::<Vec<&str>>();
    // stacks.for_each(|x| println!("{}", x.to_string()));

    let proc = content.lines().skip(empty_line_index + 1).collect::<Vec<&str>>();
    // proc.for_each(|x| println!("{}", x.to_string()));

    let stacks = build_stacks(stacks_definition);
    println!("stacks: {:?}", stacks);
    println!("========== DAY 05 ===========");
    println!("Part 1 : ");
    println!("Part 2  :");
}
