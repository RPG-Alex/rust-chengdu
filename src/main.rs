use std::io;
use std::io::Write; // For the write! macro

enum Choice {
    GoLeft,
    GoRight,
    Quit,
}

fn get_user_choice() -> Choice {
    loop {
        println!("Do you go left or right? (L/R/Q)");
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the `>` appears before the program waits for user input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().to_lowercase().as_str() {
            "l" => return Choice::GoLeft,
            "r" => return Choice::GoRight,
            "q" => return Choice::Quit,
            _ => {
                println!("Invalid choice! Please enter L, R or Q.");
                continue;
            }
        }
    }
}

fn main() {
    println!("Welcome to the dungeon!");

    loop {
        match get_user_choice() {
            Choice::GoLeft => println!("You go left and find a dead end."),
            Choice::GoRight => println!("You go right and find a treasure chest!"),
            Choice::Quit => {
                println!("You've chosen to quit the game. Goodbye!");
                break;
            }
        }
    }
}
