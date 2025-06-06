// Results for throwing errors

use std::fs::read_to_string;

fn main() {
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file: {}", err),
    }
}
