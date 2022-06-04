use std::io::{Write, Read};
use std::env;
use std::fs::*;
use std::error::Error;

fn main() {
    
    let mut args_iter = env::args();

    args_iter.next();

    match args_iter.next() {
        Some(option) => {
            match option.as_str() {
                "add" => {
                    add_note(args_iter.next());
                },
                "reset" => {
                    reset();
                },
                "remove" => {
                    remove_note(args_iter.next());
                },
                _ => {
                    help()
                }
            };
        },
        None => {
            help()
        }
    }

}

fn add_note(note: Option<String>) -> Result<(), Box<dyn Error>> {
    // do the code to add a note

    let note = match note {
        Some(t) => t,
        None => {
            return Err();// try to resolve this problem later
        }
    };

    let mut file = match File::open(".qkn.txt") {
        Ok(f) => f,
        Err(e) => {
            File::create(".qkn.txt")?
        }
    };

    file.write(String::from("nova nota").as_bytes())?;

    Ok(())
}

fn remove_note(index: Option<String>) -> Result<(), Box<dyn Error>> {

    println!("removed note x");

    Ok(())
}

fn reset() -> Result<(), Box<dyn Error>> {

    println!("Reseted list");

    Ok(())
}

fn help() {
    println!("HELP!");
}
