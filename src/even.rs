fn main() {
    let ans = is_even(11);
    print!("{}", ans);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}