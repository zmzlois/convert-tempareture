use dialoguer::Select;
use std::{f64, io};

fn main() {
    let items = vec!["Fahrenheit", "Celsius"];

    let mut is_fahrenheit: bool = true;
    let selection = Select::new()
        .with_prompt("Do you want to convert fahrenheit or celsius?")
        .items(&items)
        .interact()
        .unwrap();

    println!("You chose: {}", items[selection]);

    if items[selection] == items[0] {
        is_fahrenheit = true;
    } else if items[selection] == items[1] {
        is_fahrenheit = false;
    }

    println!("Input the degree to calculate: ");

    let mut degree = String::new();

    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to get number");

    let degree: u32 = match degree.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("It must be a number! Error: {err}");
            0
        }
    };

    if is_fahrenheit {
        println!("Requested degree: {degree}. Calculating...");
        let result = (degree as f64 - 32.0) * (5.0 / 9.0);
        println!("The requested celsius is {result}");
    } else {
        let result = (degree as f64 * 9.0 / 5.0) + 32.0;
        println!("The requested fahrenheit is {result}");
    }
}
