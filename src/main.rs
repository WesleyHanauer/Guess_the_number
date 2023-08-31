use std::io;

fn generate_random_number() -> i32 {
    let random_number = 1;
    return random_number;
}

fn main() -> io::Result<()> {
    let mut selected_number = String::new();
    let generated_number = generate_random_number();
    let stdin = io::stdin();
    println!("Pick a number between 1 and 100!");
    stdin.read_line(&mut selected_number)?;
    let selected_number_integer = selected_number.trim().parse::<i32>();
    match selected_number_integer {
        Ok(parsed_integer) => {
            if parsed_integer == generated_number {
                println!("Good job! You guessed it: {}", parsed_integer);
            } else {
                println!("Wrong number! The correct number was: {}", generated_number);
            }
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
        }
    }
    Ok(())
}