// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn return_name(name: &str) -> &str {
    name
}

fn main() {
    let first_name = return_name("Peter");
    let last_name = return_name("Dackers");
    println!("{} {}", first_name, last_name);
}
