use std::io;

fn main() {
    println!("Simple calculator (enter 'exit' to quit)");
    println!("⢀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⡀\n⣿ Simple calculator in Rust  ⣿");
    print!("⣿  (Entrer 'exit' to stop)   ⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n");

    loop {
        // ask for the first number
        println!("⣿    Enter a first number:   ⣿");
        // \n⣿                            ⣿

        let mut first_input = String::new();
        io::stdin().read_line(&mut first_input).expect("Failed to read line");
        let trimmed_first = first_input.trim();

        // Check for exit condition
        if trimmed_first.eq_ignore_ascii_case("exit") {
            println!("⣿                            ⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿");
            println!("⣿   Exiting the calculator   ⣿\n⣿          program !         ⣿");
            print!("n          Bye bye !         ⣿\n⣿                            ⣿\n⠈⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁\n\n");
            break;
        }

        // Parse the first integer
        let num1: f64 = match trimmed_first.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("⣿                            ⣿\n⣿     Invalid input.         ⣿");
                print!("⣿     Please enter a         ⣿\n⣿     valid integer.         ⣿\n");
                print!("⣿                            ⣿\n");
                continue;
            }
            
        };

        // ask for the operand
        println!("⣿     Enter an operand:      ⣿");

        let mut operator_input = String::new();
        io::stdin().read_line(&mut operator_input).expect("Failed to read line");
        let operator = operator_input.trim();
        
        
        // Print the result if the operand is ² or ^10
        // Perform calculation
        match operator { 
            "²" => {
                let temp_result = num1 * num1;
                println!("⣿    {}² = {}                ⣿\n", num1, temp_result);
            },
            "^10" => {
                let temp_result = num1 * 10.0;
                println!("⣿    {}^10 = {}              ⣿\n", num1, temp_result);
            },
            _ => {
                    // continue;
                    // ask for the second number
                    println!("⣿    Enter a second number:  ⣿");

                    let mut second_input = String::new();
                    io::stdin().read_line(&mut second_input).expect("Failed to read line");
                    let trimmed_second = second_input.trim();

                    // parse the second integer
                    let num2: f64 = match trimmed_second.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("⣿     Invalid input.         ⣿\n");
                            print!("⣿     Please enter a         ⣿\n");
                            print!("⣿     valid integer.         ⣿\n");
                            continue;
                        }
                    };

                    // Perform calculation
                    let result = match operator {
                        "+" => num1 + num2,
                        "-" => num1 - num2,
                        "*" => num1 * num2,
                        "/" => {
                            if num2 == 0.0 {
                                println!("⣿     Error: Division by 0   ⣿\n");
                                continue;
                            }
                            num1 / num2
                        }
                        "%" => num1 % num2,
                        _ => {
                            println!("⣿  Unsupported operator: '{}' ⣿\n⣿                            ⣿", operator);
                            continue;
                        }
                    };
                println!("⣿     The result is:         ⣿\n⣿     {}{}{} = {}            ⣿", num1, operator, num2, result);
                    
            }
        }
        
        

        // Display the result
        // println!("Result: {} {} {} = {}\n\n\n", num1, operator, num2, result);
        
    };
        
    println!("   _      _\n  (<      >)\n   `O,99,O`\n  //-\\__/-\\\\  Crabithmetics.");
        
        
        
        
        
        

        
    
}
