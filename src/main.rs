use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //::new() means empty, as opposed to `let guess = 5` (we want to fill it with a guess!)
    //by having mut - mutabiilitiy - the variable can be rewritten. Without mut prevents it. Similar to go using = vs := (I think that's how that worked)

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
