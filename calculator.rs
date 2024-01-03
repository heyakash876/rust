use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    println!("Enter the operator (+, -, *, /): ");
    io::stdin().read_line(&mut operator).unwrap();
    let operator = operator.trim();

    println!("Enter the second number: ");
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Unknown operator"),
    };

    println!("The result is: {}", result);
}

