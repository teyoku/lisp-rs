use std::error::Error;

mod lexer;
mod object;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");
    
    Ok(())
}
