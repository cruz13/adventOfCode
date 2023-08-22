use std::fs;

const PATH: &str = "asset/day01.input.txt";
const TOP_ELVES_NUMBER: usize = 3;

fn main() {
    let contents = fs::read_to_string(PATH).expect("Should have been able to read a file");
    //println!("{}", contents);
    let mut calories: usize = 0;
    let mut elf_counter: usize = 0;
    let mut max_calories: usize = 0;
    let mut top_three_calories = [0; TOP_ELVES_NUMBER];

    println!(" ===== DAY 01 ===== ");
    for line in contents.lines() {
        let calorie: usize = match line.trim().parse() {
            Ok(cal) => cal,
            Err(_) => {
                // There is an error if the line parsed is not a `usize`.
                // Which means that it's an empty line .. so a new elf.
                
                // if needed, we update the max values.
                if calories > max_calories{
                    max_calories = calories;
                }

                //Trying to get the smaller value from the array with iterators .. but failed:
                //with : top_three_calories.iter().position( |&x| x < calories ){
                //or with: top_three_calories.iter().enumerate().min_by(????somehting???) {
                // .. in the end, doing it with for loop :
                let mut index_of_smaller_value = 0;
                for (index, &value) in top_three_calories.iter().enumerate() {
                    if value < top_three_calories[index_of_smaller_value] {
                        index_of_smaller_value = index;
                    }
                }
                if top_three_calories[index_of_smaller_value] < calories {
                    top_three_calories[index_of_smaller_value] = calories;
                    //println!("{:?}", top_three_calories);
                }

                //current elf: updating counters
                elf_counter += 1;
                calories = 0;
                continue;
            },
        };
        calories += calorie;
    }
    let grand_total = top_three_calories[0] + top_three_calories[1] + top_three_calories[2];
    println!("Part01: It's the last elf ! (n°{}) he's is carrying {} calories (max calories for now : {})", elf_counter, calories, max_calories);
    println!("Part02: The top three elves are carrying [{:?}] calories for a total of {}.",top_three_calories, grand_total);
    println!(" ------------------ ");
}

