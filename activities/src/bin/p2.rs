// Project 2: Contact manager
//
// User stories:
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * The `id` and `name` fields are required, the `email` field is optional.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;

#[allow(dead_code)]
fn read_and_list_file_contents() -> std::io::Result<()> {
    let file = File::open("p2_data.csv")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

#[allow(dead_code)]
fn append_to_file(name: String, mail: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("p2_data.csv")
        .unwrap();
    let latest_id = provide_latest_id();
    let my_int: i32 = latest_id.parse().unwrap();
    writeln!(file, "{},{},{}", my_int + 1, name, mail)?;
    Ok(())
}

#[allow(dead_code)]
fn provide_latest_id() -> String {
    let file = File::open("p2_data.csv").expect("Cannot open file.");
    let reader = BufReader::new(file);
    let last_line = reader.lines().last()
        .expect("No last line!")
        .expect("IO error reading file.");
    let split = last_line.split(",");
    let vec: Vec<&str> = split.collect();
    let id = String::from(vec[0]);
    return id;
}

fn main() {
    /*
    match read_and_list_file_contents() {
        Ok(()) => println!("ok"),
        Err(_) => println!("error"),
    }
    */
    let name = String::from("name");
    let mail = String::from("mail");

    match append_to_file(name, mail) {
        Ok(()) => println!("ok"),
        Err(_) => println!("error"),
    }
}
