use std::io; // lib for input.

fn main() {
    let mut input = String::new();
    println!("Hour calculator type how many minutes:"); // prints the question
    io::stdin().read_line(&mut input).unwrap(); // Reads the input
   
    let number: i32 = input.trim().parse().unwrap_or(0); // Prints 0 if error occurs
    println!("so {} minutes is,{} hours", input, number / 60); // divides number by 60 cuz 60 minutes is 1 hour
}