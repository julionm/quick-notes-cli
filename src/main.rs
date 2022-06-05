use std::io::{Write, Read, Seek, SeekFrom};
use std::env;
use std::fs::*;
use std::error::Error;
use std::process;
use std::ops::Add;

fn main() {
    
    let mut args_iter = env::args();

    args_iter.next();

    match args_iter.next() {
        Some(option) => {
            match option.as_str() {
                "add" => {
                    let arg = match args_iter.next() {
                        Some(a) => a,
                        None => {
                            println!("Not enough arguments");
                            help();

                            process::exit(1)
                        }
                    };

                    if let Err(e) = add_note(arg) {
                        println!("The adding note failed: {e}");
                        process::exit(1);
                    }
                },
                "list" => {
                    list();
                },
                "reset" => {
                    if let Err(e) = reset() {
                        println!("The reset notes failed: {e}");
                        process::exit(1);
                    }
                },
                "remove" => {
                    let arg = match args_iter.next() {
                        Some(a) => a,
                        None => {
                            println!("Not enough arguments");
                            help();

                            process::exit(1);
                        }
                    };

                    if let Err(e) = remove_note(arg) {
                        println!("The remove note failed: {e}");
                        process::exit(1);
                    }
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

fn add_note(note: String) -> Result<(), Box<dyn Error>> {
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(".qkn.txt") {
        Ok(f) => f,
        Err(e) => {
            File::create(".qkn.txt")?
        }
    };

    file.seek(SeekFrom::End(0))?;

    writeln!(file, "{}", note.as_str())?;

    Ok(())
}

fn remove_note(index: String) -> Result<(), &'static str> {

    let index = match index.as_str().chars().next() {
        Some(c) => c,
        None => {
            return Err("Wrong type argument");
        }
    };

    if !index.is_numeric() {
        return Err("The index is not a number");
    }

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(".qkn.txt") {

            Ok(f) => f,
            Err(e) => {
                match File::create(".qkn.txt") {
                    Ok(f) => f,
                    Err(e) => {
                        return Err("Error trying to remove item: {e}");
                    }
                }
            }
    };

    let mut content = String::new();

    file.read_to_string(&mut content);

    let mut counter: u32 = 0;

    let content_without_line: String = content.lines().filter(|_line| {
        if counter == u32::from(index) {
            return true
        }

        counter += 1;

        false
    }).collect();

    println!("{content_without_line}");

    println!("removed note {index}");

    Ok(())
}

fn reset() -> Result<(), Box<dyn Error>> {

    println!("Reseted list");

    Ok(())
}

fn list() -> Result<(), &'static str> {

    let mut f = match OpenOptions::new().read(true).open(".qkn.txt") {
        Ok(f) => f,
        Err(e) => {
            return Err("Add some Notes first!");
        }
    };

    let mut content = String::new();

    f.read_to_string(&mut content);

    let mut counter: u32 = 0;

    let content: String = content.lines().map(|line| {
        let new_line = format!("[{counter}] {line}\n");
        counter += 1;
        new_line
    }).collect();

    println!("{content}");

    Ok(())
}

fn help() {
    println!("HELP!");
}
