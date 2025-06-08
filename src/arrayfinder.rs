use std::io;

fn main() {
    let array = ["kartik", "Tomar", "uwu"];

    println!("Please enter any number in index");

    
    loop {
        let mut index = String::new();
        
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to enter line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Index typed isnt a number");
                continue;
            }
        };

        if index < array.len() {
            let element = array[index];
            println!("The item at index {} is {}", index, element);
            break;
        } else {
            println!(
                "Invalid index! Please enter a number between 0 and {}.",
                array.len() - 1
            );
        }
    }
}
