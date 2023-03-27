// Collecting command line arguments using rust is a bit different than other languages.
// We use the std::env module to get the arguments.
// The std::env module has a function called args() which returns an iterator of the command line arguments.
// We can use the collect() method on the iterator to collect the arguments into a vector.

// The std::env module also has a function called args_os() which returns an 
//iterator of the command line arguments as OsString values.

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // let command = args[1].clone();
    // let name = "Ishan";
    // let status = "100%";

    // // println!("Command: {}", command);

    // if command == "hello" {
    //     println!("Hi {}, how are you?", name);
    // } else if command == "status" {
    //     println!("Status is {}", status);
    // } else {
    //     println!("That is not a valid command");
    // }
    let mut str = String::new();
    for (index, value) in args.iter().enumerate() {
        if index != 0{
            str.push_str(value);
            str.push_str(" ");
        }
    }
    println!("{}", str);
}