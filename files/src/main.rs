use std::fs;

fn main() {
    let file = fs::read_to_string("input")
        .expect("Failed to read input!!");

    println!("{}", file);
}
