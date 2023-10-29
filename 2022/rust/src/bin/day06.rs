use advent_of_code_2022::{get_content, DataType};

fn find_marker(data_stream: Vec<char>, marker_size: usize) -> usize {
    let mut marker = vec![' '; marker_size];
    
    for index in 0..data_stream.len() {
        marker[(index + 1) % marker_size] = data_stream[index];
        if index >= marker_size {
            let mut something = marker.clone();
            something.sort();
            something.dedup();
            if something.len() == marker_size {
                return index + 1;
            }
        }
    }
    0 // should never happen
}

fn main() {
    let content = get_content(6, DataType::Input);

    let marker_index = find_marker(content.chars().collect(), 4);
    let start_of_message_index = find_marker(content.chars().collect(), 14);

    println!("========== DAY 06 ===========");
    println!("Part 1 : {}", marker_index);
    println!("Part 2  : {}", start_of_message_index);
}
