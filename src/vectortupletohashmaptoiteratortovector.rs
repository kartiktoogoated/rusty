// take a vector that is of a tuple (string, number) convert into a hashmap
// iterate over a hashmap create iterator from it then convert to a vector
use std::collections::HashMap;

fn main() {
    // step 1 start with vector of tuples
    let vec_of_tuples = vec![(String::from("Kartik"), 20), (String::from("Harkirat"), 25)];

    // step 2 convert vector in a hashmap
    let my_map: HashMap<_,_> = vec_of_tuples.into_iter().collect();

    // Step 3: Create an iterator from the HashMap
    let iter = my_map.iter();

    // step 4 convert  the iterator back into a vector
    let vec_from_map: Vec<(String, i32)> = iter
        .map(|(k, v) | (k.clone(), *v))
        .collect();

    println!("{:?}", vec_from_map);
}