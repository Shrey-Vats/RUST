use std::io;
fn main() {
    println!("🌡️ Welcome to the CLI Temperature Converter!");
    println!("1 -> Fahrenheit to Celsius");
    println!("2 -> Celsius to Fahrenheit");

    let mut select = String::new();

    if let Err(_) = io::stdin().read_line(&mut select) {
        println!("❌ Failed to read input. Please try again.");
        return;
    }

    let sel: i32 = match select.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid selection. Enter 1 or 2.");
            return;
        }
    };

    // let sel: i32 = select.trim().parse().expect("Incorrect");

    let mut temp_input = String::new();

    match sel {
      1 =>  println!("Enter the temperature in Fahrenheit: "),
      2 =>  println!("Enter the temperature in Celsius: "),
      _ =>  {
            println!("❌ Invalid choice. Please run again with 1 or 2.");
            return;
        }
    };

    if let Err(_) = io::stdin().read_line(&mut temp_input) {
        println!("❌ Failed to read temperature.");
        return;
    };

    let temperature: f32 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invliad Input. please enter only float or number");
            return;
        }
    };

    let result = match sel {
        1 => fahrenheit_to_celsius(temperature),
        _ => celsius_to_fahrenheit(temperature)
    };

    match sel {
        1 => println!("the temperature is {:.2?}", result),
        _ => println!("the temperature is {:.2?}", result)
    }
}

fn fahrenheit_to_celsius(fa: f32) -> f32 {
    (fa - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(cel: f32) -> f32 {
    (cel * 9.0 / 5.0) + 32.0
}
