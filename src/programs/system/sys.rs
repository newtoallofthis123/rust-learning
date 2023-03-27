mod format;
mod nio;
struct Student{
    name: String,
    marks: u8,
    grade: f32,
    class: String
}

impl Student{
    fn new(name: String, marks: u8, grade: f32, class: String) -> Student{
        Student{
            name,
            marks,
            grade,
            class
        }
    }
}

fn add(){
    println!("Enter the name of the student:");
    let name = nio::input();
    println!("Enter the marks of the student:");
    let marks = nio::conv_int(nio::input());
    println!("Enter the grade of the student:");
    let grade = nio::conv_float(nio::input());
    println!("Enter the class of the student:");
    let class = nio::input();
    let _student = Student::new(name, marks, grade, class);
    println!("Student added successfully!");
}

fn view(){
    println!("Viewing students");
}

pub fn start(){
    format::welcome_screen();
    println!("So what have you decided to do?");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    choice = choice.to_string();
    let choice = choice.trim();
    let options: Vec<&str> = vec![
        "add", "view", "exit"
    ];
    if !options.contains(&choice) {
        println!("Invalid choice");
        return;
    }
    match choice {
        "add" => add(),
        // "view" => view(),
        // "exit" => exit(),
        _ => println!("Invalid choice")
    }
}