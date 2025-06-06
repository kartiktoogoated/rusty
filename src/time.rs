use chrono::{Local, Utc};

fn main() {
    // Get the current data and time in UTC
    let now = Utc::now();
    println!("Current date and time is {}", now);

    // Format the date and time 
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time is {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Local date and time is {}", local);

    // Format Local date and time
    let local_formatted = local.format("%Y-%m-%d %H:%M:%S");
    println!("Local formatted date and time is {}", local_formatted);

}