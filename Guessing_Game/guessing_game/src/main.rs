use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number program!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    println!("Input the number you want to guess:");

    // Let is used to declare a variable, and the mut indicates that this variable is mutable.
    // The String::new() means that the new function will be creating a new empty String
    // The :: means that the String is associated with new
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input) // Work similar to the Clang, but in this case we use the &mut to specify that is a mutable variable we are referencing 
        .expect("There was a failure!"); // The read_line function returns ok or error, if there was an error this function will indicate with the message defined

    println!("You guessed: {input}");
}
