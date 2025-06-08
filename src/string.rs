// created, mutated, deleting
fn main() {
    let mut name = String::from("Kartik");
    name.push_str(" Tomar");
    println!("{}", name);
    name.replace_range(10..name.len(), "");
    println!("{}", name);
}

// write a fn takes a string as an input and returs first word
// unpotimal solution

fn main() {
    let name = String::from("Hello world");
    let ans = first_word(name);
    println!("{}", ans);
}

fn first_word(str: String) -> String {
    let mut ans =  String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
     }
     return ans;
}

// Optimal solution