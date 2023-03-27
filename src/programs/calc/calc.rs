use std::io;

pub fn calc(){
    let mut choice = String::new();
    println!("Enter a choice:");
    io::stdin().read_line(&mut choice).expect("Unable to read string");
    let options: Vec<&str> = vec![
        "add", "sub", "mul", "div", "mod"
    ];
    let choice = choice.trim();
    if !options.contains(&choice) {
        println!("Invalid choice");
        return;
    }
    let mut num1 = String::new();
    // trim is used to remove the newline character
    // parse is used to convert the string to a number
    // parse can be used to convert to any type that implements the FromStr trait
    // example: let num1: i32 = num1.trim().parse().expect("Unable to parse number");
    // This is like explicitly casting the string to an integer
    println!("Enter a number:");
    io::stdin().read_line(&mut num1).expect("Unable to read string");
    let num1: i32 = num1.trim().parse().expect("Unable to parse number");
    let mut num2 = String::new();
    println!("Enter another number:");
    io::stdin().read_line(&mut num2).expect("Unable to read string");
    let num2: i32 = num2.trim().parse().expect("Unable to parse number");
    // if choice in options:
    // Using the match statement is like the switch statement in other languages
    match choice {
        "add" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "sub" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "mul" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "div" => println!("{} / {} = {}", num1, num2, num1 / num2),
        "mod" => println!("{} % {} = {}", num1, num2, num1 % num2),
        _ => println!("Invalid choice")
    }
}