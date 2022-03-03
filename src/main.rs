use rand::Rng;
use std::io;
mod words;
// We are making a hangman game
// We are using the rand crate to select a random word from the list of words
fn get_random_word() -> String {
    let mut rng = rand::thread_rng();
    let word = words::WORDS[rng.gen_range(0..words::WORDS.len())].to_string();
    word
}

fn print_rules(word: &str) {
    println!("Welcome to hangman!");
    println!(
        "You will be given a word to guess. You will have {} incorrect guesses before you lose.",
        word.len() + 2
    );
    println!("Each turn, you will input a letter. If the letter is in the word, its position will be revealed.");
    println!("If the letter is not in the word, you lose a guess.");
    println!("If you have guessed all letters, you win!");
    println!("Good luck!");
}

// This function prints the word with the guessed letters revealed
fn print_word(word: &String, guessed_letters: &Vec<char>) {
    // Print all the letters of word that are in guessed_letters
    // If the letter is not in guessed_letters, print a blank space
    // println!("Word: {}, guessed letters: {:?}", word, guessed_letters);
    for letter in word.chars() {
        if guessed_letters.contains(&letter) {
            print!("{}", letter);
        } else {
            print!("_");
        }
    }
    println!();
}

fn ask_for_letter(guessed_letters: &Vec<char>) -> char {
    // Ask the user for a letter
    // If the letter is already in guessed_letters, ask again
    // If the letter is not in guessed_letters, return the letter
    let letter: char;
    println!("Please enter a letter:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());
    // Check if input contains at least one letter
    if input.trim().chars().count() != 1 {
        println!("Please enter a single letter.");
        ask_for_letter(guessed_letters);
    }
    letter = input.trim().chars().nth(0).unwrap();
    if guessed_letters.contains(&letter) {
        println!("You already guessed that letter!");
        ask_for_letter(guessed_letters);
    }
    letter.to_uppercase().nth(0).unwrap()
}

fn check_letter(word: &String, letter: &char, lives: &mut i32) {
    // Check if the letter is in the word
    // If the letter is in the word, return true
    // If the letter is not in the word, return false
    if word.contains(*letter) {
        println!("The letter {} is in the word!", letter);
    } else {
        *lives -= 1;
        println!("The letter {} is not in the word! Lives: {}", letter, lives);
    }
}

fn check_win(word: &String, guessed_letters: &Vec<char>, lives: &i32) -> bool {
    // Check if all letters of word are in guessed_letters
    // If all letters are in guessed_letters, return true
    // If not all letters are in guessed_letters, return false

    if (*lives) <= 0 {
        println!("You lose! The word was: {}", word);
        return true;
    }
    if word.chars().all(|letter| guessed_letters.contains(&letter)) {
        println!("You win!");
        return true;
    }
    false
}

fn main() {
    let word = get_random_word();
    print_rules(&word);
    let mut guessed_letters = Vec::new();
    let mut lives = word.len() as i32 + 2;
    loop {
        let letter = ask_for_letter(&guessed_letters);
        guessed_letters.push(letter);
        check_letter(&word, &letter, &mut lives);
        print_word(&word, &guessed_letters);
        if check_win(&word, &guessed_letters, &lives) {
            break;
        }
    }
}
