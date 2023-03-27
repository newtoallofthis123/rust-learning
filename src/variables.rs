// Variables in Rust are immutable by default.
// This means that once a value is bound to a name, you can’t change that value.
// To make a variable mutable, use the mut keyword.
// So by default, all variables are const.
// But if you want to change the value of a variable, you can use the mut keyword.
// So, using mut keyword, it is like let

pub fn run(){
    let name = "Ishan";
    // name = "Joshi"; // This will throw an error
    // Now the variable name is mutable
    let mut age = 18;
    println!("The age was {}", age);
    age = 19;
    // const does exist, but it is not used much
    // A explicit type must be given to a const
    const ID: i32 = 001;
    println!("ID: {}", ID);
    // Id I feel is sort of like a global variable
    // To make it memory safe, the const must be annotated with a type
    println!("My Name is {} and I am {} old", name, age);

    // Assign multiple variables
    let (my_name, my_age) = ("Ishan", 18);
    println!("{} is {}", my_name, my_age);

    // BTW, I love that the rust-analyzer extension highlights the variables
}