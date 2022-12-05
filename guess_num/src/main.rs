use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the numer");
    loop {
    println!("enter the number");
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..100);

    io::stdin().read_line(&mut guess).expect("failed to read line");
    //let guess: u32 = guess.trim().parse().expect("please type a number");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) =>continue;
    };

    println!("you guessed {}", guess);
    match guess.cmp(&secret) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {
            println!("Hurray! you win!"),
            break;
        }
    }
    }
}
