use std::io;

fn main() {
    /*
    TODO: 
        - User input choose what temperature will input 'F' or 'C'
        - User input the number
        - Return the value of the convertion based on what the user chose
    */
    loop {
        println!("Choose an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Quit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Couldn't read the line");
        let option: u16 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid option");
                continue;
            }
        };

        match option {
            1 => {
                println!("Enter a temperature in Celcius");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Couldn't read the line");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number");
                        continue;
                    }
                };

                let convertion = c_to_f(num);

                println!("The convertion to C to F is {convertion}");
            }
            2 => {
                println!("Enter a temperature in Fahrenheit");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Couldn't read the line");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number");
                        continue;
                    }
                };

                let convertion = f_to_c(num);

                println!("The convertion to F to C is {convertion}");
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please choose again.");
            }
        }
    }
}

// Fahrenheit to Celcius
fn f_to_c(num: f64) -> f64 {
    // formula (°F − 32) × 5/9 = 0 °C
    (num - 32.0) * (5.0 / 9.0)
}

// Celcius to Fahrenheit
fn c_to_f(num: f64) -> f64 {
    // formula 0 °C × 9/5) + 32 = 32 °F
    num * (5.0 / 9.0) + 32.0
}
