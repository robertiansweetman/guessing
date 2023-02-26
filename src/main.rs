extern crate rand; // import an external crate

use std::io; //accept user input
use rand::Rng; // we can call anything in the rand crate by calling rand before it and the Rng trait defines methods for random number generators to implement
use std::cmp::Ordering; // Like Result, Ordering is another enum with variants of Less, Greater and Equal

fn main() { // main program entry point, fn with no parameters
    println!("Guess the number");

    // let n: u32 = rng.gen_range(0..=10);

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("your secret: {}", secret_number);    

    println!("Type in your guess");

    let mut guess = String::new(); // create an empty variable, has to be mutable, new() is an associated function of a string type (static method?)

    io::stdin().read_line(&mut guess) // read_line from stdin and assign it to &mut guess. io::stdin <- calls the associated function stdin() of the io part of the std library. The string argument has to be mutable so the read_line method can change the value of the guess variable. The & indicates this argument is a reference so that allows multiple parts of the code to access the data without it needing to copy the code into memory multiple times.

        .expect("Failed to read line"); // this single line is divided into 2 for readability. read_line returns an io::Result value. Rust has generic types named Result and specific version for submodules i.e. io::Result. Result types are enums with values called variants.

        // Result types are values of Ok or Err. Ok indicates that the activity was successful - inside the Ok is the successfully generated value. If it fails then Err contains info how/why it failed. If "whatever" is okay, expect will take the return value that is okay and (only) and allow you to use it

        // If you don't handle/include the .expect then rust will warn you that you haven't used the RESULT value returned from read_line 

    // secret_number has been declared as a u32 positive number so unless you convert the guess variable into one as well the program won't compile due to Rust's strong typing

    // re-using the guess variable name as a shadow of the previous value of guess, with a new one. This language feature is often usef where you want to convert one value to another type WITHOUT having to create guess and guess_string 

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    // binding the guess to the expression `guess.trim().parse()` which takes the guess value, trims whitespace off it and parses it into a u32 integer. The fact that let guess: u32 tells the parser what tye of value to chuck out the other end.

    //If the methods used on guess don't result in a u32 you get an error (via expect again), which prints an error out.

    // See below where Rust infers the type of secret_value based on guess being parsed

    println!("You guessed: {}", guess); // print the value you guessed

    // When guess is used in the compare Rust infers that &secret_number is a u32 type as well with the & pointing to the reference where the value of secret_number is.

    // very lovely how Rust does match expressions here
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("YES! YOU WIN")
    }
}