use std::{fs, error::Error, env};

pub struct Config {
    pub q : String, 
    pub fp : String,
    pub ignore_case : bool,
}

impl Config {
    pub fn new(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let ignore_case : bool = env::var("IGNORE_OK").is_ok();
        args.next(); let q = match args.next() {Some(arg) => arg,
        None => return Err("Query Missing")}; 
        let fp = match args.next() {Some(arg) => arg, None => return Err("Filepath Missing")};
        Ok(Config {q, fp, ignore_case})
    }
}

pub fn search_cs<'a>(s1 : &str, s2 : &'a str) -> Vec<&'a str> {
    let ss1 = s1.to_lowercase();
    s2.lines().filter(|x| x.to_lowercase().contains(&ss1)).collect()
}

pub fn search<'a>(s1 : &str, s2 : &'a str) -> Vec<&'a str> {
    s2.lines().filter(|x| x.contains(s1)).collect()
}

pub fn run(cfg : Config) -> Result<(), Box<dyn Error>> {
    let ct = fs::read_to_string(cfg.fp)?;
    let rt;
    if cfg.ignore_case {rt = search( &cfg.q, &ct);} else {
        rt = search_cs(&cfg.q, &ct);}
    for ll in rt {println!("{ll}");}
    Ok(())
}
