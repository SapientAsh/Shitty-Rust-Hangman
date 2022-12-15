extern crate reqwest;

use std::io;
use rand::Rng;

fn main() -> std::io::Result<()> {
    const MIN_LENGTH: usize = 5;
    let mut rng = rand::thread_rng();
    let file = reqwest::blocking::get("http://www.mieliestronk.com/corncob_lowercase.txt").expect("Couldn't retrieve word file")
    .text().expect("File could not be interpreted");
    let words: Vec<&str> = file.split("\n")
    .filter( | line | line != &"" && line.len() >= MIN_LENGTH)
    .collect();

    const MAX_FAIL: u8 = 5;
    let mut fails: u8 = 0;
    let secret: Vec<char> = words[rng.gen_range(0..words.len())]
    .chars()
    .collect();
    let mut guesses: Vec<char> = Vec::new();
    let mut print = vec!['-'; secret.len()];
     
    //while the game is running
    loop {
        //reinitialize guess
        let mut guess = String::new();

        //display word progress 
        for i in 0..secret.len() {
            if guesses.contains(&secret[i]) {
                print[i] = secret[i];
            }
        }

        //win if word is correct
        if secret == print {
            println!("Congratulations! The word was {}.", secret.iter().collect::<String>());
            break;
        }
         
        println!("{}", print.iter().collect::<String>());
        println!("You have {} remaining mistakes\n", MAX_FAIL - fails);

        //get user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
         
        let guess: char = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        guesses.push(guess);

        //penalize if input is not correct
        if !secret.contains(&guess) {
            println!("Incorrect\n");
            fails += 1;
        }
        
        //lose if too many fails
        if fails > MAX_FAIL {
            println!("Wow you're really bad at this");
            println!("The word was {}", secret.iter().collect::<String>());
            break;
        }
        
    }

    Ok(())
}
