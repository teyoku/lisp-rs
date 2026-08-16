use std::error::Error;

mod errors;
mod lexer;
mod object;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");

    Ok(())
}
