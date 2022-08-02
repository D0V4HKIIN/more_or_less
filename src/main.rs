use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Hello, world!");

	let secret:u32 = rand::thread_rng().gen_range(1..=100);

	println!("Secret is {secret}");
	loop{
		let mut guess: String = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue
		};

		
		println!("You guessed {guess}");

		match guess.cmp(&secret) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
