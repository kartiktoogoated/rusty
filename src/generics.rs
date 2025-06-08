fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![10, 20, 5, 23, 3];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['a', 'b', 'c'];
    let result = largest(&chars);
    println!("The largest char is {}", result);
}
