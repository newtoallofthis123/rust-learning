// Loops in Rust are similar to other languages.
// The main difference is that Rust has three types of loops: loop, while, and for.
// The loop keyword is used to create an infinite loop.
// The while keyword is used to create a loop that runs while a condition is true.
// The for keyword is used to create a loop that runs for a certain number of iterations.

pub fn run(){
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    for i in x.iter() {
        println!("Number: {}", i);
    }

    // While loop (FizzBuzz)
    let mut y = 1;
    while y <= 100 {
        if y % 15 == 0 {
            println!("FizzBuzz");
        } else if y % 3 == 0 {
            println!("Fizz");
        } else if y % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", y);
        }
        y += 1;
    }
    // For Range
    for i in 0..100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    // Infinite loop
    let mut z = 0;
    loop {
        z += 1;
        println!("Number: {}", z);
        if z == 20 {
            break;
        }
    }
}