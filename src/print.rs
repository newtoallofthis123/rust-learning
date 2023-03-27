// Pub means that this function is public and can be used by other modules
// Rust uses semicolons to end statements
pub fn print(){
    // Println is a macro that prints a string to the console
    println!("Hello World");
    // Use {} to print variables
    println!("{} {} is so cool", "Ishan", "Joshi");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Ishan", "India", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Ishan", activity = "Cricket");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 9, 9, 9);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}