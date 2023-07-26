use std::{env, process};

use c27_io_project::Config;

fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    // println!("Search for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = c27_io_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}