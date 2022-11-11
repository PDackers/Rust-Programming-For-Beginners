// Project 1: Interactive bill manager
//
// User stories:
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.


// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
use std::collections::HashMap;
use std::io;

// * L1: I want to add bills, including the name and amount owed.
fn add_bill(bills: &mut HashMap<&str, f64>) {
    bills.insert("dinner3", 77.25);
}

// * L1: I want to view existing bills.
fn view_bills(bills: &HashMap<&str, f64>) {
    println!("The existing bills are:");
    for (bill, amount) in bills {
        println!("{bill}: {amount}");
    }
}

fn remove_bills(bills: &mut HashMap<&str, f64>) {
    if bills.contains_key("dinner") {
        bills.remove("dinner");
    }
}

// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
fn get_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn show_options() {
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove a bill");
    println!("4. Edit a bill");
    println!("5. Exit");
}

// * Use the loop keyword to create an interactive menu.
fn main() {
    let mut bills: HashMap<&str, f64> = HashMap::new();

    // add_bill(&bills, "dinner", 84.50);

    bills.insert("dinner", 84.50);
    bills.insert("dinner2", 81.50);
    loop {
        show_options();
        view_bills(&mut bills);
        remove_bills(&mut bills);
        add_bill(&mut bills);
        view_bills(&mut bills);
        break;
    }
}
