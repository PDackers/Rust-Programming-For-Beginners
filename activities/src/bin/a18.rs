// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Customer {
    age: u8,
}

fn elligible(age: u8) -> Result<String, String> {
    if age >= 21 {
        Ok(String::from("Yay"))
    } else {
        Err(String::from("Customer not old enough."))
    }
}

fn main() {
    let customer_one: Customer = Customer { age: 39 };
    let result = elligible(customer_one.age);
    match result {
        Ok(message) => println!("{}", message),
        Err(message) => println!("{}", message),
    }
}
