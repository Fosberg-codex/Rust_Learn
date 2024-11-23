use std::io;
use rand::Rng;


fn main(){

println!("Welcome to my number guess game");
println!("Enter any number between 1 - 100");

let randomnum = rand::thread_rng().gen_range(1..=100);

let mut guessnum = String::new();

io::stdin().read_line(&mut guessnum).expect("Error in reading your guess number");
// println!("your guessed number is {}",guessnum);

let guessnumb: i32 = match guessnum.trim().parse(){
Ok(num) => num
Err(_) => { println!("Please enter a valid number")
return }
}

println!("Yoour guessed numnber is {randomnum}");

if randomnum == guessnumb {
    println!("you won!! with machine guess {guessnum}");
} else {
    println!("You Lost!! Machine says guess is  {guessnum}, try again");
}



}
