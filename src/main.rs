use std::io::{self, Write};

static mut TOTAL_RESULT: f64 = 0.0;

fn main() {
    println!("Choose mode:");
    println!("1. Standard Calculator");
    println!("2. RPN Calculator");

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();

    match mode.trim() {
        "1" => {
            standard_calculator();
        }
        "2" => {
            rpn_calculator();
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn standard_calculator() {
    println!("Standard Calculator");

    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => {
                unsafe {
                    TOTAL_RESULT += result;
                    println!("Current total: {}", TOTAL_RESULT);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn evaluate_expression(input: &str) -> Result<f64, &'static str> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut result = 0.0;
    let mut operation: Option<&str> = None;

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            match operation {
                Some(op) => {
                    result = match op {
                        "+" => result + num,
                        "-" => result - num,
                        "*" => result * num,
                        "/" => {
                            if num == 0.0 {
                                return Err("Cannot divide by zero");
                            }
                            result / num
                        }
                        _ => return Err("Unknown operator"),
                    };
                }
                None => {
                    result = num;
                }
            }
        } else if "+-*/".contains(token) {
            operation = Some(token);
        } else {
            return Err("Wrong input");
        }
    }

    Ok(result)
}

fn rpn_calculator() {
    println!("RPN Calculator");

    let mut stack: Vec<f64> = Vec::new();

    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();

        for token in tokens {
            if let Ok(num) = token.parse::<f64>() {
                stack.push(num);
            } else {
                if stack.len() < 2 {
                    println!("Wrong input");
                    continue;
                }

                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();

                let result = match token {
                    "+" => operand1 + operand2,
                    "-" => operand1 - operand2,
                    "*" => operand1 * operand2,
                    "/" => {
                        if operand2 == 0.0 {
                            println!("Cannot divide by zero");
                            stack.push(operand1);
                            stack.push(operand2);
                            continue;
                        }
                        operand1 / operand2
                    }
                    _ => {
                        println!("Wrong input");
                        stack.push(operand1);
                        stack.push(operand2);
                        continue;
                    }
                };

                stack.push(result);
            }
        }

        if stack.len() == 1 {
            let final_result = stack.pop().unwrap();
            unsafe {
                TOTAL_RESULT += final_result;
                println!("Current total: {}", TOTAL_RESULT);
            }
        } else if stack.len() > 1 {
            println!("Wrong input");
        }
    }
}
