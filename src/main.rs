use std::fs;

fn main() {
    let token = fs::read_to_string("token.txt").expect("token from file");
    println!("{}", token);
}
