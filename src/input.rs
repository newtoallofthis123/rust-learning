use std::io;

pub fn run(){
    println!("Enter your name: ");
    let mut name = String::new();
    // The & is a reference to the variable name
    // The &mut is a mutable reference to the variable name
    // This is sort os like the call by reference in C++
    // It is also like javascript, .catch() and .then()
    // The .expect() is like the try catch block in javascript
    // Instead of using std::io::stdin().read_line(&mut name).expect("Failed to read line");
    // We can use io::stdin().read_line(&mut name).expect("Failed to read line");
    // Because we have used the use std::io; statement
    // This is like the import statement in python
    // We can also use the use std::io::stdin; statement
    // This is like the from statement in python
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello {}", name);
}