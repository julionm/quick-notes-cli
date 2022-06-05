use std::{
    io::{Write, Read, Seek, SeekFrom},
    fs::*,
    error::Error,
    env
};


pub struct QknStorage {
    filename: String
}

impl QknStorage {
    pub fn new(filename: String) -> QknStorage {
        QknStorage { filename }
    }

    pub fn add_note(&self, note: String) -> Result<(), Box<dyn Error>> {
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(".qkn.txt") {
            Ok(f) => f,
            Err(_e) => {
                File::create(&self.filename)?
            }
        };

        file.seek(SeekFrom::End(0))?;

        writeln!(file, "{}", note.as_str())?;

        Ok(())
    }

    pub fn remove_note(&self, index: String) -> Result<(), &'static str> {

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
                Err(_e) => {
                    match File::create(&self.filename) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("{e}");
                            return Err("Error trying to create the file");
                        }
                    }
                }
        };

        let mut content = String::new();

        match file.read_to_string(&mut content) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                return Err("Error when trying to read the file");
            }
        };

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

    pub fn reset(&self) -> Result<(), Box<dyn Error>> {

        println!("Reseted list at: {}", self.filename);

        Ok(())
    }

    pub fn list(&self) -> Result<(), &'static str> {

        let mut f = match OpenOptions::new().read(true).open(&self.filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                return Err("Add some Notes first!");
            }
        };

        let mut content = String::new();

        match f.read_to_string(&mut content) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                return Err("Error trying to open the file")
            }
        };

        let mut counter: u32 = 0;

        let content: String = content.lines().map(|line| {
            let new_line = format!("[{counter}] {line}\n");
            counter += 1;
            new_line
        }).collect();

        println!("{content}");

        Ok(())
    }

}

pub fn help() {
    println!("HELP!");
}

pub fn run(mut args_iter: env::Args, qkn_storage: QknStorage) -> Result<(), &'static str> {
    match args_iter.next() {
        Some(option) => {
            match option.as_str() {
                "add" => {
                    let arg = match args_iter.next() {
                        Some(a) => a,
                        None => {
                            return Err("Not enough arguments")
                        }
                    };

                    if let Err(e) = qkn_storage.add_note(arg) {
                        eprintln!("{e}");
                        return Err("The adding note failed, verify if the correct syntax was used with help}")
                    }
                },
                "list" => {
                    match qkn_storage.list() {
                        Ok(a) => a,
                        Err(e) => {
                            eprintln!("{e}");
                            return Err("Unexpected Error when listing the file content")
                        }
                    }
                },
                "reset" => {
                    if let Err(e) = qkn_storage.reset() {
                        eprintln!("{e}");
                        return Err("Error on reseting the notes, have you created any?")
                    }
                },
                "remove" => {
                    let arg = match args_iter.next() {
                        Some(a) => a,
                        None => {
                            return Err("Not enough arguments");
                        }
                    };

                    if let Err(e) = qkn_storage.remove_note(arg) {
                        eprintln!("{e}");
                        return Err("Failed in removing the note")
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

    Ok(())
}
