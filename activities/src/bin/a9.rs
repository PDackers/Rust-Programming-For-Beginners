// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn cartesian_coord(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let (cartesian_x, cartesian_y) = cartesian_coord(5, 4);
    if cartesian_y > 5 {
        println!("greater than!");
    } else if cartesian_y < 5 {
        println!("smaller than!");
    } else {
        println!("equal to 5!");
    }
}
