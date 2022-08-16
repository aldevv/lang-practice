use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let ans = rand::thread_rng().gen_range(1..10);
    let mut first_try = true;
    loop {
        let mut guess = String::new();
        if !first_try {
            println!("Try again!");
        }
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&ans) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal=> {
                println!("You win!");
                return;
            }
        }

        // this is the turbofish operator ::<>
        // let guess_parsed = guess.trim_end().parse::<i32>()
        //     .expect("please put a valid num");

        // if guess == ans {
        //     println!("You Won! answer: {}", ans);
        //     return
        // }
        first_try = false;
    }
}
