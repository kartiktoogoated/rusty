fn main() {
    let name = String::from("kartik");
    let len = get_str_len(name);
    print!("The length of the sting is{}", len)
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}