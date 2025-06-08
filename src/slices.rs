fn main() {
    let mut word = String::from("Hello World");
    let ans = first_word(&word);
    println!("{}", ans);
}

fn  first_word(word: &String) -> &str {
    let mut space_index = 0;
    for i in word.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index+1;
    }
    return &word[0..space_index];
}

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