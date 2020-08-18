fn get_line() -> String {
    // Get one line of input from stdin and strip newline character
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.len() - 1);
    return line;
}

fn get_operation() -> String {
    // Get one character for operation from stdin
    println!("Enter operation:");
    return get_line();
}

fn get_numbers() -> Vec<f64> {
    // Get two numbers from stdin
    let mut output_vec: Vec<f64> = Vec::new();
    let mut stop_flag: bool = false;
    while !stop_flag {
        println!("Enter value (Press \"#\" to stop):");
        let line: String = get_line();
        if line == "#" {
            stop_flag = true;
        } else {
            let output_int: f64 = line.parse().unwrap();
            output_vec.push(output_int)
        }
    }
    return output_vec;
}

fn main() {
    // Main program loop
    println!(
        "Possible Operations with n numbers: +, -, *, /
Possible operations with two numbers (additional numbers ignored): %, <<, >>, ^"
    );
    let operation: String = get_operation();
    let input: Vec<f64> = get_numbers();
    match operation.as_str() {
        "+" => {
            println!(
                "\nResult: {}",
                input[1..].iter().fold(input[0], |accumulator, value| accumulator + value)
            );
        }
        "-" => {
            println!(
                "\nResult: {}",
                input[1..].iter().fold(input[0], |accumulator, value| accumulator - value)
            );
        }
        "*" => {
            println!(
                "\nResult: {}",
                input[1..].iter().fold(input[0], |accumulator, value| accumulator * value)
            );
        }
        "/" => {
            println!(
                "\nResult: {}",
                input[1..].iter().fold(input[0], |accumulator, value| accumulator / value)
            );
        }
        "%" => println!("\nResult: {}", input[0] % input[1]),
        "<<" => println!("\nResult: {}", (input[0] as u32) << (input[1] as u32)),
        ">>" => println!("\nResult: {}", (input[0] as u32) >> (input[1] as u32)),
        "^" => println!("\nResult: {}", (input[0] as i32).pow(input[1] as u32)),
        _ => panic!("Unimplemented operation!"),
    }
}
