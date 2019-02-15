use std::io;
use std::process;
fn main() {
	println!("Welcome to my Triangle Sum Theorem Calculator! Only positive numbers will be shown because it isn't possible to have negetive numbers as an input or output shown with the Triangle Sum Theorem.");
	loop {
		let mut choice = String::new();
		println!("\nNow select the action you'd like to do by entering the number that's next to the action.");
		println!("1. 180° - a° - b° = c°");
		println!("2. Exit program");
		io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
			eprintln!("Application Error: {}", err);
			process::exit(1);
		});
		let choice = choice.trim() as &str;
		match choice {
			"1" | "1." => {
				println!("Enter the value of a.");
				let sum = convert();
				println!("Enter the value of b.");
				let choice = convert();
				println!("\nThe solution is:");
				println!("180° - {}° - {}° = {}°", sum.abs(), choice.abs(), 180.0 - sum.abs() - choice.abs());
			}
			"2" | "2." => {
				println!("Goodbye!");
				process::exit(0);
			}
			&_ => {
				eprintln!("You entered an invalid value.");
				process::exit(1);
			}
		}
	}
}
fn convert()->f64 {
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	let choice:f64 = choice.trim().parse().unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	return choice;
}