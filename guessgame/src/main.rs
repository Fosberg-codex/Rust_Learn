use std::io;
use rand::Rng;

fn main(){
    println!("The best number bet game");

    println!("Guess a random number");

    let mut guessnum = String::new();

    io::stdin()
        .read_line(&mut guessnum)
        .expect("Please enter a valid number");

    let randomnum = rand::thread_rng().gen_range(1..=100);

    let guessnumb:u32 = guessnum.trim().parse().expect("Please enter a valid number");

    // let guessnumb:u32 = match guessnum.trim().parse(){
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Error please enter a valid number");
    //         return;
    //     }
    // };

    if guessnumb == randomnum{
        println!("You Won!! with machine number{randomnum}");
    } else{
        println!("You Lost!! with machine number being {randomnum}");
    }

    // println!("The you guess is {guessnum}");
    // println!("Machine number is {randomnum}");
}

