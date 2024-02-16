use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn call(&self) -> f64 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => a / b,
        }
    }
}

fn operation(a: f64, b: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(Operation::Add(a, b).call()),
        "-" => Ok(Operation::Subtract(a, b).call()),
        "*" => Ok(Operation::Multiply(a, b).call()),
        "/" => {
            if b <= 0.0 {
                Err(String::from(
                    "The second number could not be negative or cero",
                ))
            } else {
                Ok(Operation::Divide(a, b).call())
            }
        }
        _ => Err(String::from("Operation not allowed")),
    }
}

fn get_input() -> Result<String, String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn prompt() -> Result<(f64, f64, String), String> {
    println!("Entry the first number");
    let input_a: f64 = match get_input()?.parse() {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    println!("Entry the second number");
    let input_b: f64 = match get_input()?.parse() {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    println!("Entry the operation: ADD: +, SUBTRACT: -, MULTIPLY: *, DIVIDE: /");
    let input_op: String = match get_input() {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    Ok((input_a, input_b, input_op))
}

fn main() {
    match prompt() {
        Ok(result) => {
            match operation(result.0, result.1, result.2.as_str()) {
                Ok(t) => println!("Result: {:?}", t),
                Err(e) => println!("Error: {:?}", e),
          }
        },
        Err(e) => println!("{:?}", e),
    }
}
