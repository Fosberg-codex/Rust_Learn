use std::io;


fn main(){

println!("Welcome to my number guess game");
println!("Enter any number between 1 - 100");

let mut guessnum = String::new();

io::stdin().read_line(&mut guessnum).expect("Error in reading your guess number");
println!("your guessed number is {}",guessnum);

}
