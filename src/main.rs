use std::{
    env,
    process
};

use qkn::{QknStorage, help, run};

fn main() {
    
    let mut args_iter = env::args();

    args_iter.next();

    let qkn_storage = QknStorage::new(String::from("/tmp/.qkn.txt"));

    if let Err(e) = run(args_iter, qkn_storage) {
        if e.starts_with("Not enough arguments") {
            help();
        } else {
            eprintln!("{e}");
        }

        process::exit(1)
    }

}
