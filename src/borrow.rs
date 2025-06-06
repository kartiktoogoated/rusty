fn main () {
    let s1 = String::from("Kartik");
    do_something(&s1);
    println!("number is {}", s1 );
}

fn do_something(s2: &String) {
    println!("{}", s2);
}

fn main () {
    let mut s1 = String::from("Kartik");
    do_something(&mut s1);
    println!("{}", s1 );
}

fn do_something(s2: &mut String) {
    s2.push_str(" Tomar");
    println!("{}", s2);
}
// Either 1 mutable or any number of immutable refrences
