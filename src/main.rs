fn main() {
    let word = String::from("hi i am kartik");

    let mut index = 0;
    for i in word.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    let ans = &word[0..index];
    println!("{}", ans);
}