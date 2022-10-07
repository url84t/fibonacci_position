use std::io;

fn main() {
    loop {
        println!(
            "Please enter the position of the fibonacci number you would like to see (up to the 186th position) or type quit"
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            break;
        }

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a valid entry");
                continue;
            }
        };

        if guess > 186 {
            println!("please choose a position of 186 or less");
        }

        println!(
            "the fibonacci number at position {} is: {}",
            guess,
            fibonacci_by_position(guess)
        );
    }
}

fn fibonacci_by_position(position: u64) -> u128 {
    let mut current = 1;
    let mut last = 0;
    let mut hold;
    let mut iterator = 1;

    while iterator != position {
        // iterate the iterator
        iterator += 1;

        hold = current;
        current = last + current;
        last = hold;
    }

    last
}
