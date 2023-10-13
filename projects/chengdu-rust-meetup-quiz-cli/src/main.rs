// Import the Rng trait from the rand crate for random number generation.
use rand::Rng;
// Import standard library modules: input/output and time measurement.
use std::{io, time::Instant};

// Define a struct for a question.
struct Question {
    text: String,
    options: Vec<String>,
    correct_answer: usize,
}

// Define a struct for the question bank.
struct QuestionBank {
    questions: Vec<Question>,
    asked_questions: Vec<usize>,
}

// Implement functionality for the QuestionBank struct.
impl QuestionBank {
    // Method to get a random question from the bank.
    fn get_random_question(&mut self) -> Option<&Question> {
        // Generate a list of indices for questions that haven't been asked.
        let available_questions: Vec<usize> = (0..self.questions.len())
            .filter(|index| !self.asked_questions.contains(index))
            .collect();

        // If there are no questions left, return None.
        if available_questions.is_empty() {
            return None;
        }

        // Randomly select an index from the available questions.
        let random_index = rand::thread_rng().gen_range(0..available_questions.len());
        let question_index = available_questions[random_index];
        // Add the selected question to the asked questions list.
        self.asked_questions.push(question_index);
        // Return the selected question.
        Some(&self.questions[question_index])
    }
}

// Entry point of the program.
fn main() {
    // Print the game introduction.
    println!("#### Welcome to the dungeon! ####");
    println!("#### Answer the Questions Correctly to proceed! ####");
    println!("####  Make the wrong move and face the consequences! ####");

    // Initialize the player's lives.
    let mut lives: i8 = 3;

    // Initialize the question bank.
    let mut question_bank = initialize_question_bank();

    // Main game loop.
    loop {
        // Get a random question to ask.
        if let Some(question) = question_bank.get_random_question() {
            println!("{}", question.text);
            // Print each option for the question.
            for option in &question.options {
                println!("{}", option);
            }

            // Read player's answer with a time limit.
            let mut answer = String::new();
            let answer_time = 10;
            println!("You have {} seconds to answer!", answer_time);
            let start_time = Instant::now();
            io::stdin().read_line(&mut answer).unwrap();
            // Convert the answer to lowercase for case-insensitive comparison.
            answer = answer.trim().to_lowercase();

            // Check if the time limit has been exceeded.
            if start_time.elapsed().as_secs() > answer_time {
                println!("Time is up! You lose a life!");
                answer = String::from("_");
                lives -= 1;
                println!("Current lives: {}\n", lives);
                continue;
            }

            // Determine the correct answer based on the question's correct_answer field.
            let correct_option = match question.correct_answer {
                0 => "a",
                1 => "b",
                2 => "c",
                3 => "d",
                // This should never be reached but is needed for the match to be exhaustive.
                _ => "",
            };

            // Check if the player's answer is correct.
            if answer != correct_option {
                println!("Wrong Answer! You lose a life.");
                lives -= 1;
                println!("Current lives: {}\n", lives);
                // Exit the game if the player is out of lives.
                if lives <= 0 {
                    println!("You are out of lives. Goodbye (RIP)!");
                    break;
                }
            } else {
                println!("Correct! Proceeding onward through the Oxidungeon!");
                println!("Current lives: {}\n", lives);
            }
        } else {
            // Player has answered all questions correctly.
            if lives > 0 {
                println!("Congratulations! You've answered all the questions and are now a Rust Master!");
            }
            break;
        }
    }
}

// Function to initialize the question bank with a set of questions.
fn initialize_question_bank() -> QuestionBank {
    // List of predefined questions.
    let questions = vec![
        Question {
            text: String::from("Which web framework in Rust is inspired by Flask and Express.js?"),
            options: vec![
                String::from("A) Rocket"),
                String::from("B) Actix"),
                String::from("C) Iron"),
                String::from("D) Warp"),
            ],
            correct_answer: 0,
        },
        Question {
            text: String::from("What crate in Rust allows you to interface with SQL databases using an ORM-like fashion?"),
            options: vec![
                String::from("A) Diesel"),
                String::from("B) R2D2"),
                String::from("C) Serde"),
                String::from("D) Tokio"),
            ],
            correct_answer: 0,
        },
        Question {
            text: String::from("Which game engine is written in Rust?"),
            options: vec![
                String::from("A) Godot"),
                String::from("B) Unreal Engine"),
                String::from("C) Unity"),
                String::from("D) Amethyst"),
            ],
            correct_answer: 3,
        },
        Question {
            text: String::from("Which Rust crate allows for easy 2D game development?"),
            options: vec![
                String::from("A) ggez"),
                String::from("B) Piston"),
                String::from("C) Rayon"),
                String::from("D) Mio"),
            ],
            correct_answer: 0,
        },
        Question {
            text: String::from("Which crate is NOT related to asynchronous programming in Rust?"),
            options: vec![
                String::from("A) Futures"),
                String::from("B) Tokio"),
                String::from("C) Rocket"),
                String::from("D) Async-std"),
            ],
            correct_answer: 2,
        },
        Question {
            text: String::from("Which website offers interactive Rust exercises to enhance your Rust skills?"),
            options: vec![
                String::from("A) Rust Playground"),
                String::from("B) Rustlings"),
                String::from("C) Crates.io"),
                String::from("D) Rustup.rs"),
            ],
            correct_answer: 1,
        },
        Question {
            text: String::from("Chengdu is famous for being the home to which animal?"),
            options: vec![
                String::from("A) Lions"),
                String::from("B) Elephants"),
                String::from("C) Giant Pandas"),
                String::from("D) Tigers"),
            ],
            correct_answer: 2,
        },
        Question {
            text: String::from("Which of the following is a popular dish originating from Chengdu?"),
            options: vec![
                String::from("A) Hot Pot"),
                String::from("B) Sushi"),
                String::from("C) Tacos"),
                String::from("D) Pasta"),
            ],
            correct_answer: 0,
        },
    ];

    // Return a QuestionBank instance with the list of questions and an empty list of asked questions.
    QuestionBank {
        questions,
        asked_questions: Vec::new(),
    }
}

