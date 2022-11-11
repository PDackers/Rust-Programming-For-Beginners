// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

enum Status {
    Active,
    Deactivated,
}

struct Employee {
    department: EmployeeType,
    employed: Status,
}

fn permission_to_enter(employee: &Employee) -> Result<(), String> {
    match employee.employed {
        Status::Deactivated => return Err(String::from("Not currently working here.")),
        _ => (),
    }

    match employee.department {
        EmployeeType::Maintenance => Ok(()),
        EmployeeType::Managers => Ok(()),
        EmployeeType::Marketing => Ok(()),
        _ => Err(String::from("Invalid department.")),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = permission_to_enter(employee)?;
    println!("Access granted.");
    Ok(())
}

fn main() {
    let becky: Employee = Employee {
        department: EmployeeType::Managers,
        employed: Status::Active,
    };

    let access = print_access(&becky);

    match access {
        Ok(()) => println!("User entered the building."),
        Err(e) => println!("{}", e),
    }
}
