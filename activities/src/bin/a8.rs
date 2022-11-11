// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Fruity,
    Sour,
    Sweet,
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn print_information(drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("Fruity"),
        Flavor::Sour => println!("Sour"),
        Flavor::Sweet => println!("Sweet"),
    };
    println!("oz: {:?}", drink.ounce);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        ounce: 6.4,
    };
    print_information(sweet);
}
