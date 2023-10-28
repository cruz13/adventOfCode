use advent_of_code_2022::{get_content, DataType};
use regex::Regex;

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

fn get_top_crates(stacks: &Vec<Vec<char>>) -> Vec<&char> {
    let mut top_crates: Vec<&char> = Vec::new();
    stacks.iter().for_each(|x| top_crates.push(x.last().unwrap()));
    top_crates
}

fn parse_procedure(raw_procedure: Vec<&str>) -> Vec<Vec<usize>> {
    let mut procedure = vec![];
    let re = Regex::new(r"(?m)^move (?<crate_nb>\d+) from (?<crate_origin>\d+) to (?<crate_destination>\d+)$").unwrap();
    
    for line in raw_procedure {
        for (_, [crate_nb, crate_origin, crate_destination]) in re.captures_iter(line).map(|c| c.extract()) {
            procedure.push(vec![crate_nb.parse::<usize>().unwrap(), crate_origin.parse::<usize>().unwrap(), crate_destination.parse::<usize>().unwrap()]);
        }
    }
    procedure
}

fn move_crates_with_cratemoved9000(stacks: &Vec<Vec<char>>, procedure: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    let mut end_stacks = stacks.clone();
    for proc in procedure {
        for _ in 0..proc[0] {
            // println!("moving crate {:?} from {:?} to {:?}", end_stacks[proc[1] - 1].last().unwrap(), end_stacks[proc[1] - 1], end_stacks[proc[2] - 1]);
            let temp = end_stacks[proc[1] - 1].pop().unwrap(); 
            end_stacks[proc[2] - 1].push(temp);
        }
    }
    end_stacks
}

fn move_crates_with_cratemoved9001(stacks: &Vec<Vec<char>>, procedure: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    let mut end_stacks = stacks.clone();
    for proc in procedure {
        let nb = proc[0];
        let size = end_stacks[proc[1]-1].len();
        let mut temp = end_stacks[proc[1] - 1].drain(size - nb ..).collect();
        end_stacks[proc[2] - 1].append(&mut temp);
    }
    end_stacks
}

fn main() {
    let content = get_content(5, DataType::Input);

    let empty_line_index = content.lines().position(|x| x.is_empty()).unwrap();

    let stacks_definition = content.lines().take(empty_line_index).collect::<Vec<&str>>();
    println!("--- Stacks are : --------------");
    stacks_definition.iter().for_each(|x| println!("{:?}", x));

    let proc = content.lines().skip(empty_line_index + 1).collect::<Vec<&str>>();
    // println!("--- Movement procedure is : ---");
    // proc.iter().for_each(|x| println!("{:?}", x));
    println!("-------------------------------");

    let stacks = build_stacks(stacks_definition);
    println!(" - top crates at begining are: {:?}", get_top_crates(&stacks));
    // println!("stacks: {:?}", stacks);

    let procedures = parse_procedure(proc);
    // procedures.iter().for_each(|x| println!("{:?}", x));

    let end_stacks = move_crates_with_cratemoved9000(&stacks, &procedures);

    println!(" - top crates at the end are: {:?}", get_top_crates(&end_stacks));
    println!("stacks: {:?}", end_stacks);
   
    let end_stacks_9001 = move_crates_with_cratemoved9001(&stacks, &procedures);
    println!(" - top crates at the end with 9001 are: {:?}", get_top_crates(&end_stacks_9001));
    println!("stacks: {:?}", end_stacks_9001);

    println!("========== DAY 05 ===========");
    println!("Part 1 : {}", get_top_crates(&end_stacks).into_iter().collect::<String>());
    println!("Part 2  : {}", get_top_crates(&end_stacks_9001).into_iter().collect::<String>());
}
