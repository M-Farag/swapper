use std::env;
use std::fs;
use regex::Regex;

fn main() {
    // ToDo
    // [x] Accept arguments
    // [x] Read the file
    // [x] Parse the file & swap the letters
    // [x] Write the file

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let origin_letter = &args[2];
    let new_letter = &args[3];

    let mut contents = fs::read_to_string(file_path).unwrap();
    let re = Regex::new(&origin_letter).unwrap();
    contents = re.replace_all(&contents, new_letter).to_string();

    fs::write(file_path, contents).unwrap();

}
