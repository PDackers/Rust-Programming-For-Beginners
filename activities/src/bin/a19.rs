// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u8> = HashMap::new();
    let mut counter: u8 = 0;

    stock.insert("Chairs".to_string(), 5);
    stock.insert("Beds".to_string(), 3);
    stock.insert("Tables".to_string(), 2);
    stock.insert("Couches".to_string(), 0);

    for (key, value) in stock.iter() {
        if *value == 0_u8 {
            println!("{}: out of stock", key);
        } else {
            println!("{}: {}", key, value);
        }
    }

    for quantity in stock.values() {
        counter = counter + quantity;
    }

    println!("Total items in stock: {}", counter);
}
