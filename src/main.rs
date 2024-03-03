use std::io;

fn main() {
    println!("Input 1 for F-C and 2 for C-F.");

    loop {
        let mut selection = String::new();

        io::stdin().read_line(&mut selection).expect("Failed to read line.");

        let selection: f64 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You selected {selection}");

        if selection == 1.0 {
            println!("Input the fahrenheit temperature you would like to convert to Celsius:");

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Error.");

            let input: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            let result: f32 = (input - 32.0) / 1.8;

            println!("The result is: {result}.");
            break;
        }
        if selection == 2.0 {
            println!("Input the Celsius temperature you would like to convert to Fahrenheit:");

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Error.");

            let input: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            let result: f32 = ((9.0/5.0) * input) + 32.0;

            println!("The result is: {result}.");
            break;
        }
        else {
            println!("Input 1 for F-C and 2 for C-F.");
        }
    }
}