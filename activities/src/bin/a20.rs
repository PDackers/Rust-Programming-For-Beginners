// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_power_state(state: PowerStates) {
    match state {
        PowerStates::Off => println!("Offline."),
        PowerStates::Sleep => println!("Entering sleepmode."),
        PowerStates::Reboot => println!("Rebooting."),
        PowerStates::Shutdown => println!("Shutting down."),
        PowerStates::Hibernate => println!("Starting hibernate."),
    }
}

fn get_input() -> io::Result<String> {
    println!("Input what action you want to take:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase().to_owned())
}

fn main() {
    match get_input() {
        Ok(buffer) => match buffer.as_str() {
            "off" => print_power_state(PowerStates::Off),
            "sleep" => print_power_state(PowerStates::Sleep),
            "reboot" => print_power_state(PowerStates::Reboot),
            "shutdown" => print_power_state(PowerStates::Shutdown),
            "hibernate" => print_power_state(PowerStates::Hibernate),
            _ => println!("Not a valid input."),
        },
        Err(msg) => println!("{}", msg),
    }
}
