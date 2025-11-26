# Rust Wordle CLI

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/Type-CLI-blue)
![Status](https://img.shields.io/badge/Project-Toy%20Project-8a2be2)

A small Wordle inspired CLI game written in Rust.

It picks a secret word from `words.txt`, then you try to guess it in a limited number of attempts, getting feedback after each guess.

## Requirements
* Rust toolchain (install via rustup)

## Run it
```bash
git clone https://github.com/Fabio1909/Rust_Wordle_CLI
cd Rust_Wordle_CLI
cargo run
```

## How to play
- Run the program.
- Type a guess (typically a 5 letter word).
- Use the feedback to refine your next guesses until you find the secret word or run out of attempts.

## Word list

The dictionary lives in words.txt (one word per line). Edit it to add or remove allowed words.  ￼

## Notes

This is a toy project for learning Rust and building a simple terminal game.
