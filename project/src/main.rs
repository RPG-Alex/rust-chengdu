use std::io;
use std::io::Write; 

//this is how you call a crate's methods
use rand::Rng;

/// This enum represents all valid inputs for game
enum Choice {
    GoLeft,
    GoRight,
    Invalid,
    Quit,
}

///This function evaluates the user input and returns the correct choice enum
fn get_user_choice() -> Choice {
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

fn main() {
    println!("#### Welcome to the dungeon! ####\n#### This is a simple text based game ####\n#### follow the prompts to play! ####");
    
    //instantiate a rand thread_rng instance
    let mut rng = rand::thread_rng();

    //track lives using u8
    let mut lives :i8 = 1;

    loop {
        let choice = get_user_choice();
        let roll: u8 = rng.gen_range(1..10);
        match choice {
            Choice::GoLeft => {
                if roll < 3 {
                    println!("You go left and find a treasure chest and gain one life!");
                    lives += 1;
                } else if roll > 3 && roll <= 7 {
                    println!("You go left and find a dead end.");
                } else {
                    if lives > 0 {
                        println!("You go left and meet a terrible monster and lose a life!");
                        lives -= 1;
                    } else {
                        println!("You go left and meet a terrible monster and lose a life!");
                        print!("You are out of lives. Good bye!");
                        break;
                    }
                }
            }
            Choice::GoRight => {
                if roll > 7 {
                    println!("You go right and find a treasure chest and gain one life!");
                    lives += 1;
                } else if roll < 7 && roll >= 3 {
                    println!("You go right and find a dead end.");
                } else {
                    if lives > 0 {
                        println!("You go right and meet a terrible monster and lose a life!");
                        lives -= 1;
                    } else {
                        println!("You go right and meet a terrible monster and lose a life!");
                        print!("You are out of lives. Good bye!");
                        break;
                    }
                }
            }
            Choice::Invalid => {
                println!("Invalid choice! Please enter L, R or Q.");
                continue;
            }
            Choice::Quit => {
                println!("You've chosen to quit the game. Goodbye!");
                break;
            }
        }
        println!("Lives remaining: {lives}");
    }
}
