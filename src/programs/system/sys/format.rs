use std::process::Command;

pub fn welcome_screen(){
    let welcome_message = Command::new("pyfiglet")
    .args(&["Student MarkUP"]).output().expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&welcome_message.stdout));
    println!("Menu:\nAdd-> Add a student\nView-> View all students\nExit-> Exit the program");
}