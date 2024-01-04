// Define the EmployeeType enum
#[derive(Debug)]
enum EmployeeType {
    JuniorEngineer,
    SeniorEngineer,
}

// Define the Employee struct
#[derive(Debug)]
struct Employee {
    name: String,
    salary: u32,
    id: u32,
    employee_type: EmployeeType,
}

// Function to assign salary based on employee type
fn assign_salary(employee_type: &EmployeeType) -> u32 {
    match employee_type {
        EmployeeType::JuniorEngineer => 50000,
        EmployeeType::SeniorEngineer => 60000,
    }
}

fn main() {
    // Create a new Employee
    let employee = Employee {
        name: String::from("John Doe"),
        salary: 0, // This will be updated
        id: 1,
        employee_type: EmployeeType::JuniorEngineer,
    };

    // Update the salary based on the employee type
    let updated_salary = assign_salary(&employee.employee_type);

    // Print the updated salary
    println!("The updated salary is: {}", updated_salary);
}
