extern crate alloc;

use serde_derive::Deserialize;
use toml;
use alloc::string::{String, ToString};
use std::fs;
use std::process::exit;

#[derive(Deserialize)]
struct Data{
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    polpath: String,
}

fn main() {
    let filename = "config.toml";
    
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Couldn't read file {}", filename);  
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    
    let pol_path = data.config.polpath + "\\usr\\all\\";
    let mut buffer = String::new();

    println!("Which file 1 or 2");
    std::io::stdin().read_line(&mut buffer);
    
    if buffer.trim() == "1"{
        fs::copy(pol_path.to_string() + "char1.bin", pol_path.to_string() + "login_w.bin");
        
    }
    else if buffer.trim() == "2"{
        fs::copy(pol_path.to_string() + "char2.bin", pol_path.to_string() + "login_w.bin");
    }
    else {
        println!("WRONG FILE NUMBER ENTERED")
    }
    
}

