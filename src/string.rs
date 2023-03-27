// Immutable String - let name = "Ishan";
// Mutable String - let mut name = String::from("Ishan");

pub fn run(){
    let mut y = String::from("Hello");
    println!("Length: {}", y.len());
    // Push a char
    y.push('W');
    // Push a string
    y.push_str("orld!");
    // Capacity in bytes
    println!("Capacity: {}", y.capacity());
    // Check if empty
    println!("Is Empty: {}", y.is_empty());
    // Contains
    println!("Contains 'World' {}", y.contains("World"));
    // Replace
    println!("Replace: {}", y.replace("World", "There"));
    // Loop through string by whitespace
    for word in y.split_whitespace(){
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
    // Has alot of methods, check the docs
    // https://doc.rust-lang.org/std/string/struct.String.html
    // https://doc.rust-lang.org/std/primitive.str.html
}