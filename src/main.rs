use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    println!("\nGuess the number between 0 and 100!");

    loop {
	    println!("Please input your guess. A new number every time!\n");

	    let secret_number = rand::thread_rng().gen_range(1,101);

	    let mut guess = String::new(); //::new() means empty, as opposed to `let guess = 5` (we want to fill it with a guess!)
	    //by having mut - mutabiilitiy - the variable can be rewritten. Without mut prevents it. Similar to go using = vs := (I think that's how that worked)

	    io::stdin().read_line(&mut guess)
	        .expect("Failed to read line");
	    //From docs: The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.


	    // this below fixes issue with string input from console, but need int to match int of secret_number
	    let guess: u32 = match guess.trim().parse() {
	        // .expect("Please type a number!");
	        Ok(num) => num,
	        Err(_) => continue,
	    };

	    println!("You guessed: {}", guess);

	    // add some drama, then the big reveal
	    sleep(Duration::from_millis(1000));
	    println!("The secret number is: {}", secret_number);

	    // now let's do some comparisons
	    sleep(Duration::from_millis(1000));
	    if guess == secret_number {
	    	println!("YOU DID IT");
	    } else {
	    	println!("YOU'RE SUPER DUMB. YOU COULDN'T EVEN GUESS {} ?!\n", secret_number);
	    }

	    // and we can also do a comparison with .cmp
	    sleep(Duration::from_millis(2000));
	    match guess.cmp(&secret_number) {
	        Ordering::Less => {
	        	sleep(Duration::from_millis(2000));
	        	println!("UGH, TOO SMALL! WHY DO YOU THINK SO SMALL???\nYOU WERE OFF BY {}", secret_number-guess);
	        	sleep(Duration::from_millis(3000));
	        	println!("\nYOU HAVE TO KEEP PLAYING UNTIL YOU GUESS THE RIGHT NUMBER. HURRY UP THIS IS TAKING TOO LONG.\n");
	        	sleep(Duration::from_millis(3000));
	        }
	        Ordering::Greater => {
	        	sleep(Duration::from_millis(2000));
	        	println!("WAY TOO BIG! WHY IS YOUR EGO SO INFLATED???\nYOU WERE OFF BY {}", guess-secret_number);
	        	sleep(Duration::from_millis(3000));
	        	println!("\nYOU HAVE TO KEEP PLAYING UNTIL YOU GUESS THE RIGHT NUMBER. HURRY UP THIS IS TAKING TOO LONG.\n");
	        	sleep(Duration::from_millis(3000));
	        }
	        Ordering::Equal => {
	        	sleep(Duration::from_millis(2000));
	        	println!("You win! I can't believe it. You're finally free.");
	        	break;
	        }
	    }
	}
}
