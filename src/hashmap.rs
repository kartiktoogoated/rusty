use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("Kartik"), 20);
    users.insert(String::from("Hakirat"), 21);

    let first_user_age = users.get("Kaartik");

    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found"),
    }
}

use std::collections::HashMap;

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn main() {
    let input_vec = vec![(String::from("Kartik"), 22), (String::from("Harkirat"), 23)];
    let hm = group_values_by_keys(input_vec);
    println!("{:?}", hm)
}
