use std::fs::read_to_string;

fn main() {
    for line in read_to_string("src/example").unwrap().lines() {
        println!("{}", line);
    }
}
