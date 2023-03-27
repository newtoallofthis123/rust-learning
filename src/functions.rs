// Function is a block of code that is called by name
// Functions can be declared in any scope, including global scope
// Functions can be passed data to operate on (parameters) and can optionally return data (the return value)
// Functions can be declared to only be called from within the same module, the same crate, or anywhere in the program

fn greetings(name: &str) {
    println!("Hello {}", name);
}

pub fn run(){
    greetings("Ishan");
    greetings("John");
}