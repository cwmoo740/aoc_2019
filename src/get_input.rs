use std::path::Path;
use std::fs;

fn get_file_path(day: usize) -> String {
    format!("./data/day_{}.txt", day)
}

fn file_exists(day: usize) -> bool {
    let str_path = get_file_path(day);
    let path = Path::new(&str_path);
    path.is_file()
}

fn open_file(day: usize) -> String {
    let path = get_file_path(day);
    fs::read_to_string(path).unwrap()
}

pub fn main(day: usize) -> String {
    if !file_exists(day) {
        panic!("input for day {} does not exist", day);
    }
    open_file(day)
}
