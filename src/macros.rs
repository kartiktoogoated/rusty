/**
 * Macros in rust
 * a macro is like a shortcut that writes code for you so you dont have to repeat 
 * yourself again and again
 * like if you need one thing in your code that are getting used oftenly you just create a macro of it and call it again somewhere as a function.
 * Without a macro:
println!("Hello, Kartik!");
println!("Hello, Kartik!");
println!("Hello, Kartik!");
With a macro:
greet!();
greet!();
greet!();

🤯 Why not just use functions?

Because macros can do crazy powerful things that functions can’t — like:
Take any kind of input (not just fixed types).
Generate new variables or code at compile time.
Work with Rust syntax itself.
 */

macro_rules! greet {
    () => {
        println!("Hello, Kartik Youre learning rust macros")
    };
}

fn main() {
    greet!();
}

let a = vec![1, 2, 3];

// this is a macro internally it creates
let a = {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v
};