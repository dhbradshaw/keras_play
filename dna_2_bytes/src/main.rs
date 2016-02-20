extern crate phrases;

use phrases::english::greetings;
use phrases::japanese;
use phrases::dna_2_bytes;

fn main() {
    println!("Hello in English: {}", greetings::hello());
	println!("Hello in informal English: {}", greetings::hello_2());
	println!("Hello in dna_2_bytes: {}", dna_2_bytes::hello());
    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
