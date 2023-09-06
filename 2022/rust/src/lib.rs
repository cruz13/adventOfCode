use std::fs;

pub enum DataType{
    Input,
    Exemple,
}

pub fn get_content(day: usize, data_type: DataType) -> String {
    let day_formatted = format!("{:02}", day);
    let data_type = match data_type {
        DataType::Input => "input",
        DataType::Exemple => "exemple",
    };
    let file_path = format!("asset/day{}.{}.txt", day_formatted, data_type);
    fs::read_to_string(file_path).expect("Should have been hable to read file _f..")
}
