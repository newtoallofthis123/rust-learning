use std::io;

pub fn calc(){
    let mut choice = String::new();
    println!("Enter a choice:");
    io::stdin().read_line(&mut choice).expect("Unable to read string");
    let options = [
        "add", "sub", "mul", "div", "mod"
    ];
    // if choice in options:
    // Using the match statement is like the switch statement in other languages
}