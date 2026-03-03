use std::io;

const MIN_FAHRENHEIT: f64 = -459.67;
const MIN_CELSIUS: f64 = -273.15;

fn main() {
    println!("Temperature conversion program: Fahrenheit (°F) <-> Celsius (°C) v0.1.0");

    loop {
        println!("\nChoose option:");
        println!("1. Fahrenheit (°F) -> Celsius (°C)");
        println!("2. Celsius (°C) -> Fahrenheit (°F)");
        println!("0. Exit\n");
        println!("Please, enter your choice:");

        let Ok(option) = read_line() else {
            println!("\nExiting...");
            break;
        };

        let option: u8 = if let Ok(num) = option.trim().parse() {
            num
        } else {
            update_terminal();
            println!("\nInvalid option! Try again.");
            continue;
        };

        match option {
            0 => {
                println!("\nExiting...");
                break;
            }

            1 => {
                if convert_flow(
                    MIN_FAHRENHEIT,
                    "Fahrenheit",
                    "Celsius",
                    'C',
                    fahrenheit_to_celsius,
                )
                .is_err()
                {
                    update_terminal();
                    println!("\nInvalid temperature! Try again.");
                }
            }

            2 => {
                if convert_flow(
                    MIN_CELSIUS,
                    "Celsius",
                    "Fahrenheit",
                    'F',
                    celsius_to_fahrenheit,
                )
                .is_err()
                {
                    update_terminal();
                    println!("\nInvalid temperature! Try again.");
                }
            }

            _ => {
                update_terminal();
                println!("\nInvalid option! Try again.");
            }
        }
    }
}

fn read_line() -> Result<String, ()> {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        return Err(());
    }

    // Handling Ctrl + D and empty input
    if buffer.trim().is_empty() {
        return Err(());
    }

    Ok(buffer)
}

fn convert_flow(
    absolute_zero: f64,
    from_name: &str,
    to_name: &str,
    unit_label: char,
    formula: fn(f64) -> f64,
) -> Result<(), ()> {
    println!("\nEnter temperature value in {from_name}:");
    let input = read_line()?;

    let temperature: f64 = if let Ok(num) = input.trim().parse() {
        num
    } else {
        return Err(());
    };
    if temperature < absolute_zero {
        return Err(());
    }

    let result = formula(temperature);
    println!("\nYour temperature in {to_name}: {result:.2} °{unit_label}");
    Ok(())
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 9.0 / 5.0 + 32.0
}

fn update_terminal() {
    println!("\x1B[2J\x1B[1;1H");
}
