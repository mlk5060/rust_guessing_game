extern crate rand;
extern crate term;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let mut terminal = term::stdout().unwrap();
	terminal.fg(term::color::BLUE).unwrap();
	println!("\n==========================================");
    println!("===== Welcome to 'Guess the Number'! =====");
    println!("==========================================");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
    	terminal.reset().unwrap();
	    println!("\n- Please input your guess (1-100).");

	    let mut guess = String::new();
	    
	    io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line");

	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => {
	    		terminal.fg(term::color::RED).unwrap();
	    		println!("ERROR: Please input a number\n");
	    		continue
	    	}
	    };

	    match guess.cmp(&secret_number) {
	        Ordering::Less => {
	        	terminal.attr(term::Attr::Dim).unwrap();
	        	println!("Too small!")
	        },
	        Ordering::Greater => {
	        	terminal.attr(term::Attr::Bold).unwrap();
	        	println!("Too big!")
	        },
	        Ordering::Equal => {
	        	terminal.fg(term::color::BRIGHT_GREEN).unwrap();
	        	println!("You win!");
	        	terminal.reset().unwrap();
	        	break;
	        }
	    }
	}
}
