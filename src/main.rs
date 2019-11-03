#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let hoge = "aaa";
    println!("You guessed: {}", hoge);

    let a = [1, 2, 3, 4, 5];
    let index = 0;

    let element = a[index];

    println!("The value of element is: {}", element);
    rocket::ignite().mount("/", routes![index]).launch();
}

fn guess_game() {
    loop {
        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let secret_number = rand::thread_rng().gen_range(1, 11);
        println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}
