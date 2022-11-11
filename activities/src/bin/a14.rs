// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: u32,
    name: String,
    fav_color: String,
}

fn print(name: &str, color: &str) {
    println!("name: {}, likes the color: {}.", name, color);
}

fn main() {
    let people = vec![
        Person {
            age: 8,
            name: String::from("Red"),
            fav_color: String::from("red"),
        },
        Person {
            age: 10,
            name: String::from("Green"),
            fav_color: String::from("green"),
        },
        Person {
            age: 18,
            name: String::from("Blue"),
            fav_color: String::from("blue"),
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name, &person.fav_color);
        }
    }
}
