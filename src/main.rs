use std::io; //accept user input

fn main() { // main program entry point, fn with no parameters
    println!("Guess the number");

    println!("Type in your guess");

    let mut guess = String::new(); // create an empty variable, has to be mutable, new() is an associated function of a string type (static method?)

    io::stdin().read_line(&mut guess) // read_line from stdin and assign it to &mut guess. io::stdin <- calls the associated function stdin() of the io part of the std library. The string argument has to be mutable so the read_line method can change the value of the guess variable. The & indicates this argument is a reference so that allows multiple parts of the code to access the data without it needing to copy the code into memory multiple times.

        .expect("Failed to read line"); // this single line is divided into 2 for readability. read_line returns an io::Result value. Rust has generic types named Result and specific version for submodules i.e. io::Result. Result types are enums with values called variants.

        // Result types are values of Ok or Err. Ok indicates that the activity was successful - inside the Ok is the successfully generated value. If it fails then Err contains info how/why it failed. If "whatever" is okay, expect will take the return value that is okay and (only) and allow you to use it

        // If you don't handle/include the .expect then rust will warn you that you haven't used the RESULT value returned from read_line 

    println!("You guessed: {}", guess); // print the value you guessed
}