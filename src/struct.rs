struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user  = User {
        first_name: String::from("Kartik"),
        last_name: String::from("Tomar"),
        age: 32,
    };
    print!("{}", user.last_name)
}