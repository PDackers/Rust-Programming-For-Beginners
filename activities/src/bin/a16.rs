// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Assignment {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let becky: Assignment = Assignment {
        name: String::from("Becky"),
        locker: Some(324),
    };
    let _angela: Assignment = Assignment {
        name: String::from("Angela"),
        locker: None,
    };

    println!("Student name: {}", becky.name);
    match becky.locker {
        Some(num) => println!("Locker number: {}", num),
        None => println!("No locker number"),
    }
}
