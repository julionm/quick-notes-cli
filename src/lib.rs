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
            .open(&self.filename) {
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
            .open(&self.filename) {

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

        let index_int: u32 = match index.to_digit(10) {
            Some(n) => n,
            None => 0
        };

        let line_count = content.lines().count();
        let content_without_line: String = content.lines()
                .enumerate()
                .filter_map(|(i, line)| {
                    if i != (index_int as usize) {
                        if i == line_count - 1 {
                            return Some(format!("{}", line));
                        } else {
                            return Some(format!("{}\n", line));
                        }
                    }

                    None
                })
                .collect();

        match remove_file(&self.filename) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return Err("Unexpected error on removing note");
            }
        };

        match self.add_note(content_without_line) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return Err("Error on writing the new text");
            }
        };

        Ok(())
    }

    pub fn reset(&self) -> Result<(), &str> {

        match remove_file(&self.filename) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return Err("Unexpected error on removing note");
            }
        };

        match File::create(&self.filename) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return Err("Failed in reseting notes storage");
            }
        };

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

        let content: String = content.lines().enumerate().map(
                |(i, line)| format!("[{i}] {line}\n")
            ).collect();

        println!("{content}");

        Ok(())
    }

}

pub fn help() {
    println!("Quick Notes CLI\n");
    println!("To run the commands use qkn and some of the following options:\n");
    println!("  help, h - show this help command");
    println!("  add, a - add a net note");
    println!("  remove, rm - remove the note with the corresponding index");
    println!("  list, l - list all the notes with indexes");
    println!("  reset - erase all the notes\n");
    println!("Command example: qkn add \"My First Note\"");
}

pub fn run(mut args_iter: env::Args, qkn_storage: QknStorage) -> Result<(), &'static str> {
    match args_iter.next() {
        Some(option) => {
            match option.as_str() {
                "add" | "a" => {
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
                "list" | "l" => {
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
                "remove" | "rm" => {
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
                    help();
                }
            };
        },
        None => {
            help();
        }
    }

    Ok(())
}
