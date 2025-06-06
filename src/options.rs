/**
 * Options- null pointer values (return none/null/nil)
in ts
    function Sum(): number | null {
        .......
    } (either null or the value)
 * Result - Error handling
 */
enum CustomOption {
    Some(i32),
    None,
}

 fn main() {
    let index = find_first_a(String::from("Kartik"));

    match index {
        CustomOption::Some(value) => println!("{}", value),
       CustomOption::None => println!("{}", "A is not found")
    }
}


 fn find_first_a(s: String) -> CustomOption {

    for (index,char) in s.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
 }