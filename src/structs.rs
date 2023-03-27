// Structs in rust are used to create custom data types.
// Structs are sort of like objects in other languages.

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name) // Doesn't print only returns
    }
}

pub fn run(){
    let c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    let p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    println!("Person: {}", p.full_name());
}