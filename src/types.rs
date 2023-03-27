/*
Primitive Types
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples - Fixed length, heterogeneous
Arrays - Fixed length, homogeneous
*/

// Rust is a statically typed language, which means that it 
//must know the types of all variables at compile time, 
//however, the compiler can usually infer what type we want to use based on the 
//value and how we use it. 
// In that way, rust is remaniscent of python, but it is not dynamic.
// Rust is a compiled language, so it is not like python, where the type is
// checked at runtime.

pub fn run(){
    let x = 1; // By default, it is i32
    let y = 2.5; // By default, it is f64
    let z: i64 = 4545454545;
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);
    // Boolean
    let is_active = true;
    // Get boolean from expression
    let is_greater = 10 > 5;
    // Char
    let a1 = 'a';
    let face = '\u{1F600}'; // Emoji
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}