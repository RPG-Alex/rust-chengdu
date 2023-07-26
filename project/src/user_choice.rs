use std::io;
use std::io::Write; 

/// This enum represents all valid inputs for game
pub enum Choice {
    GoLeft,
    GoRight,
    Invalid,
    Quit,
}

///This function evaluates the user input and returns the correct choice enum
pub fn get_user_choice() -> Choice {
        println!("~~Do you go left or right? (L/R/Q)");
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the `>` appears before the program waits for user input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        //This matches the choice after formatting and taking a slice
        match choice.trim().to_lowercase().as_str() {
            "l" => Choice::GoLeft,
            "r" => Choice::GoRight,
            "q" => Choice::Quit,
            _ => Choice::Invalid,
        }
}