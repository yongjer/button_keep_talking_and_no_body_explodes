use std::io::{self, Write};

fn main() {
    println!("Welcome to the Bomb Defusal Assistant!");

    loop {
        let button_color = ask_question("What color is the button? (bl/w/y/r/other)");
        let button_text = ask_question("What text is on the button? (abort/detonate/hold/other)");
        
        if button_color == "bl" && button_text == "abort" {
            hold_button();
            break;
        }

        if button_text == "detonate" {
            let battery_count = ask_number("How many batteries are on the bomb?");
            if battery_count >= 2 {
                println!("Press and immediately release the button.");
                break;
            }
        }

        if button_color == "w" {
            let car_indicator = ask_yes_no("Is the CAR indicator lit?");
            if car_indicator {
                hold_button();
                break;
            }
        }

        let battery_count = ask_number("How many batteries are on the bomb?");
        let frk_indicator = ask_yes_no("Is the FRK indicator lit?");
        if battery_count >= 3 && frk_indicator {
            println!("Press and immediately release the button.");
            break;
        }

        if button_color == "y" {
            hold_button();
            break;
        }

        if button_color == "r" && button_text == "hold" {
            println!("Press and immediately release the button.");
            break;
        }

        hold_button();
        break;
    }
}

fn ask_question(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn ask_number(prompt: &str) -> u32 {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    loop {
        print!("{} (yes/no): ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please answer yes or no."),
        }
    }
}

fn hold_button() {
    println!("Hold the button.");
    let strip_color = ask_question("What color is the strip? (bl/w/y/other)");
    let release_digit = match strip_color.as_str() {
        "bl" => "4",
        "w" => "1",
        "y" => "5",
        _ => "1",
    };
    println!("Release when the countdown timer has a {} in any position.", release_digit);
}
