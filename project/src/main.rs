use std::io;
use std::io::Write; 

//this is how you call a crate's methods
use rand::Rng;

/// This enum represents all valid inputs for game
enum Choice {
    GoLeft,
    GoRight,
    Quit,
}

fn get_user_choice() -> Choice {
    loop {
        println!("~~Do you go left or right? (L/R/Q)");
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
    println!("#### Welcome to the dungeon! ####\n#### This is a simple text based game ####\n#### follow the prompts to play! ####");

    let mut rng = rand::thread_rng();

    let mut lives = 1;

    loop {
        let choice = get_user_choice();
        let roll = rng.gen_range(1..10);
        match choice {
            Choice::GoLeft => {
                if roll < 2 {
                    println!("You go left and find a treasure chest and gain one life!");
                    lives += 1;
                } else if roll > 2 && roll <= 6 {
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
                if roll > 8 {
                    println!("You go right and find a treasure chest and gain one life!");
                    lives += 1;
                } else if roll < 8 && roll >= 4 {
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
            Choice::Quit => {
                println!("You've chosen to quit the game. Goodbye!");
                break;
            }
        }
        println!("Lives remaining: {lives}");
    }
}
