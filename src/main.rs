use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    const PLANETS: [&str; 8] = [
        "mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune",
    ];
    const MAX_FAIL: u8 = 5;
    let mut fails: u8 = 0;
    let secret: Vec<char> = PLANETS[rng.gen_range(0..PLANETS.len())].chars().collect();
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
            println!(
                "Congratulations! The word was {}.",
                secret.iter().collect::<String>()
            );
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
            break;
        }
    }
}
