use rust_ml::{improve_network, test_network, train_network};

pub mod activations;
pub mod data;
pub mod matrix;
pub mod network;

fn main() {
	let command = rprompt::prompt_reply(
		"What would you like to do ('train', 'improve','test','exit')?\n -> ",
	)
	.expect("Failed to read input");

	match command.as_str() {
		"train" => train_network(),
		"improve" => improve_network(),
		"test" => test_network(),
		"exit" => println!("Goodbye!"),
		_ => println!("Invalid command! Try again!"),
	}
}
