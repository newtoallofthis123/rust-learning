// Tuples in rust are fixed length, heterogeneous

pub fn run(){
    let person: (&str, &str, i8) = ("Ishan", "India", 21);
    // The .0, .1, .2 is like the index in python
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}