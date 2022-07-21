use std::{ io, cmp::Ordering };
use rand::Rng;

pub fn comparing_the_guess_to_the_secret_number() {
    println!("[{FILE_NAME}][5] :: Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("[{FILE_NAME}][7] :: The secret number is: {secret_number}");

    loop {
        println!("[{FILE_NAME}][10] :: Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("[{FILE_NAME}][21] :: You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("[{FILE_NAME}][24] :: Too small!"),
            Ordering::Greater => println!("[{FILE_NAME}][25] :: Too big!"),
            Ordering::Equal => {
                println!("[{FILE_NAME}][27] :: You win!");
                break;
            }
        }
    }
}

static FILE_NAME: &str = "guess_demo.rs";
