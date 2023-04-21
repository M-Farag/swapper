use std::fs;
// https://docs.rs/regex/latest/regex/index.html#
use regex::Regex;
use swapper::Arguments;

fn main() {
    // todo: 
    // [x] Accept arguments
    // [x] Read the file
    // [x] Parse the file & swap the letters
    // [x] Write the file

    let args = Arguments::new();

    let mut contents = fs::read_to_string(args.file_path()).unwrap();
    let re = Regex::new(args.origin_string()).unwrap();
    contents = re.replace_all(&contents, args.new_string()).to_string();

    fs::write(args.file_path(), contents).unwrap();

}
