use std::fs;

const PATH: &str = "asset/day1.input.txt";


fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string(PATH).expect("Should have been able to read a file");
    //println!("{}", contents);
    let mut calories: usize = 0;
    let mut elf_counter: usize = 0;
    let mut max_calories: usize = 0;

    for line in contents.lines() {
        let calorie: usize = match line.trim().parse() {
            Ok(cal) => cal,
            Err(_) => {
                //previous elf:
                println!("Elf n°{} is carrying {} calories (max calories for now: {})",elf_counter, calories, max_calories);

                //current elf: updating counters
                elf_counter += 1;
                if calories > max_calories{
                    max_calories = calories;
                }
                calories = 0;
                continue;
            },
        };
        calories += calorie;
        //println!("Found {} cals", calorie);
    }
    println!("Last elf n°{} is carrying {} calories (max calories for now : {})", elf_counter, calories, max_calories);
}

