use std::{env, process};

use rustGREP::Config;

fn main() {
    let cfg : Config = Config::new( env::args()).unwrap_or_else(|err : &str| {
        println!("{err}"); process::exit(1)});
    format!("Searching for {} in {}", cfg.q, cfg.fp);
    if let Err(e) = rustGREP::run(cfg) {
        println!("Error : {e}"); process::exit(1)
    }
}


