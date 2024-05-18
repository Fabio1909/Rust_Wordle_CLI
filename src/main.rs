use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("*** WORDLE! ***\n");

    let secret_word: Result<Option<String>, std::io::Error> = read_file();

    let secret_word = match secret_word {
        Ok(Some(word)) => {
            println!("The secret word is: {}", word);
            word
        }
        Ok(None) => {
            println!("No secret word found in the file.");
            return;
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            return;
        }
    };

    let secret_word = secret_word.trim().to_lowercase();
    let sv: Vec<char> = secret_word.chars().collect();
    println!("--------------------------------");


    loop {
        let mut guess = String::new();

        while guess.trim().len() != 5{
            guess = String::new();
            println!("Guess the secret word: ");
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        }

        let guess = guess.trim().to_lowercase();
        let v: Vec<char> = guess.chars().collect();

        let eval: Vec<char> = check_words(&sv, &v);

        if eval == sv{
            println!("Pattern match: {} !!!!!", guess);
            println!("* You win! *");
            break;
        } else {
            let result_string: String = eval.clone().into_iter().collect();
            print!("Pattern match: {}", result_string);
            print!(", try again! \n");
            //std::mem::drop(v);
            //std::mem::drop(eval);
            //std::mem::drop(result_string);
            println!("--------------------------------");
        }

    }
}


fn check_words(v:&Vec<char>, g: &Vec<char>) -> Vec<char> {
    let mut res1: Vec<char> = vec!['-', '-', '-', '-', '-'];
    for i  in 0..5 {
        if v[i] == g[i]{
            res1[i] = v[i];
        }
    }
    res1
}

fn read_file() -> io::Result<Option<String>> {
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);

    let mut words = Vec::new();
    for line in reader.lines() {
        let line = line?;  // Properly handle the Result<String, io::Error>
        words.push(line);
    }
    
    let mut rng = thread_rng();
    // Using clone to return an owned String
    let word = words.choose(&mut rng).cloned();
    Ok(word)
}