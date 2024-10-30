use core::str;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{stdout, Read, Write},
};

pub enum GuessResult {
    TooSmall,
    TooBig,
    Win,
}

// Separate the game logic from the main function
// So that we can test the game logic without running the main function
pub struct GuessGameContext {
    secret_number: u32,
    guess_history: Vec<u32>,
}

impl GuessGameContext {
    fn new(secret_number: u32) -> GuessGameContext {
        GuessGameContext {
            secret_number,
            guess_history: Vec::new(),
        }
    }

    pub fn new_with_random_secret_number() -> GuessGameContext {
        let secret_number = rand::thread_rng().gen_range(1..100);
        GuessGameContext::new(secret_number)
    }

    pub fn guess(&mut self, guess: u32) -> GuessResult {
        self.guess_history.push(guess);
        self.compare_guess(guess)
    }

    fn compare_guess(&self, guess: u32) -> GuessResult {
        match guess.cmp(&self.secret_number) {
            Ordering::Less => GuessResult::TooSmall,
            Ordering::Greater => GuessResult::TooBig,
            Ordering::Equal => GuessResult::Win,
        }
    }

    pub fn get_guess_count(&self) -> usize {
        self.guess_history.len()
    }
}

fn parse_input(input: &str) -> Result<u32, std::num::ParseIntError> {
    #[cfg(debug_assertions)]
    {
        println!("DEBUG: input is {:?}", input);
    }

    input.trim_end_matches('\n').trim().parse()
}

fn main() {
    println!("[!] Welcome to the Guess Game!");
    println!("[!] The secret number is between 1 and 100, exclusive.");

    let mut game = GuessGameContext::new_with_random_secret_number();

    #[cfg(debug_assertions)]
    {
        println!("DEBUG: The secret number is {}", game.secret_number);
    }

    // max value of u32 is 4294967295, so 16 bytes is enough
    // This avoids heap allocation and is more efficient
    let mut input_buf: [u8; 16] = [0; 16];

    loop {
        print!("[?] Please input your guess: ");

        // We didn't print a newline, so we need to flush stdout
        stdout().flush().expect("Failed to flush stdout!");

        input_buf.fill(0); // clear the buffer

        // restrict the scope of mutable borrow
        let read_bytes: usize;
        {
            // used for read_vectored. I want to leave at least 1 byte for null terminator
            let mut buf_slice = [std::io::IoSliceMut::new(&mut input_buf[..15])];

            read_bytes = std::io::stdin()
                .read_vectored(&mut buf_slice)
                // this fails usually when it's unable to open stdin, so panic is reasonable here
                .expect("[!] Failed to read input!");

            debug_assert!(read_bytes > 0);
            debug_assert!(read_bytes <= input_buf.len());
        }

        // also, avoids heap allocation, to improve performance
        let guess = match parse_input(unsafe { str::from_utf8_unchecked(&input_buf[..read_bytes]) })
        {
            Ok(guess) => guess,
            Err(_) => {
                println!("[!] Invalid input, please input a number!");
                continue;
            }
        };

        match game.guess(guess) {
            GuessResult::TooSmall => println!("[!] Too small!"),
            GuessResult::TooBig => println!("[!] Too big!"),
            GuessResult::Win => {
                println!("[!] You win! - The secret number is {}", guess);
                println!(
                    "[!] Your guess history: {:?}, count {}",
                    game.guess_history,
                    game.get_guess_count()
                );

                match game.get_guess_count() {
                    1 => println!("[!] You are a genius!"),
                    2..=4 => println!("[!] You are very lucky!"),
                    10.. => println!("[!] Good luck next time!"),
                    _ => (),
                }

                break;
            }
        }
    }
}
