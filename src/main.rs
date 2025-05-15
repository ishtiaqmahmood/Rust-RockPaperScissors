use rand::Rng;
use std::io;

fn main() {
    let choices = ["rock", "paper", "scissors"];

    println!("Welcome to Rock, Paper, Scissors!");
    println!("Enter your choice (rock, paper, scissors):");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let user_choice = user_input.trim().to_lowercase();

    if !choices.contains(&user_choice.as_str()) {
        println!("Invalid choice! Please enter rock, paper, or scissors.");
        return;
    }

    let computer_choice_index = rand::thread_rng().gen_range(0..3);
    let computer_choice = choices[computer_choice_index];

    println!("You chose: {}", user_choice);
    println!("Computer chose: {}", computer_choice);

    match (user_choice.as_str(), computer_choice) {
        (u, c) if u == c => println!("It's a tie!"),
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
            println!("You win!")
        }
        _ => println!("Computer wins!"),
    }
}
