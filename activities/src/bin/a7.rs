// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Green,
    Yellow,
}

fn display(color: Colors) {
    match color {
        Colors::Red => println!("Red"),
        Colors::Blue => println!("Blue"),
        Colors::Green => println!("Green"),
        Colors::Yellow => println!("Yellow"),
    }
}

fn main() {
    let choice1 = Colors::Red;
    let _choice2 = Colors::Blue;
    let _choice3 = Colors::Green;
    let _choice4 = Colors::Yellow;
    display(choice1);
}
