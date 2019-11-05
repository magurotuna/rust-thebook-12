extern crate rust_thebook_12;

use std::env;
use std::process;

use rust_thebook_12::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_thebook_12::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
