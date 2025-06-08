fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("Kartik");
    {
        let str2 = String::from("Tomar");
        longest_str = longest(&str1, &str2);
        println!("{}", longest_str);
    }
    println!("{}", longest_str);
}

// struct with lifetimes
struct User<'a> {
    name: &'a str
}

fn main() {
    let first_name = String::from("Kartik");
    let user = User {name: &first_name};

    println!("The name of the user is {} ", user.name)
}

use std::fmt::Display;

fn longest_with_an_arguement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
 ) -> &'a str
where
    T: Display,
{
    println!("Announcement, {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("Kartik");
    let y = String::from("Tomar");
    let ann = String::from("Kartik is goat");
    println!("{}", longest_with_an_arguement(&x, &y, ann));
}