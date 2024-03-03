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

            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            let result: f64 = (input - 32.0) / 1.8;

            println!("The result is: {result}.")
        }
    else {
        println!("Input 1 for F-C and 2 for C-F.");
    }
    }
}
