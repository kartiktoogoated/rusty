trait  Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("my name is {}, and age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Kartik"),
        age: 20,
    };
    
    println!("{}", user.summarize());
}