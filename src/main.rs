use std::{io, env, cmp::Ordering};
use rand::Rng;

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    
    let test = 3;

    match test {
        1 => guess_the_number(),
        2 => get_from_array(),
        3 => tuples_use(),
        _ => println!("Invalid call!!"),
    }
}

fn guess_the_number() {
    println!("== Guess the number! ==");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_from_array() {
    let arr = [1, 2, 3, 4, 5, 6];

    println!("Please enter your array index");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read the line!");

    let index:usize = index.trim().parse().expect("Not a number!");

    let element = arr[index];

    println!("The value of the selected element at {index} is : {element}")
}

fn tuples_use() {
    // Tuple declaration
    let details = ("Saman", 24, 1.657, 72);
    let tup: (isize, f64, u8) = (500, 3.1428, 12);

    let (name, age, height, weight) = details;
    let weight: f64 = weight as f64;
    let bmi = weight / (height * height);
    println!("\nResult of tuple destructuring in Rust,");
    println!("{name} is {age} years old and his BMI is {bmi:.1}\n");

    let tup_0 = tup.0;
    let tup_1 = tup.1;
    let tup_2 = tup.2;
    println!("Another way is to use dot annotation.So,");
    println!("tup.0 = {tup_0}");
    println!("tup.1 = {tup_1}");
    println!("tup.2 = {tup_2}");
}