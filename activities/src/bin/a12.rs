// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    height: i32,
    width: i32,
    depth: i32,
    weight: i32,
    color: Color,
}

enum Color {
    Green,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Green => println!("green"),
            Color::Red => println!("red"),
        }
    }
}

impl ShippingBox {
    fn new() -> Self {
        Self {
            height: 10,
            width: 10,
            depth: 12,
            weight: 210,
            color: Color::Red,
        }
    }
    fn print(&self) {
        self.color.print();
        println!("{:#?} {:#?} {:#?} {:#?}", &self.height, &self.width, &self.depth, &self.weight);
    }
}

fn main() {
    let new_box = ShippingBox::new();
    new_box.print();
}
