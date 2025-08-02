extern crate alloc;

use serde_derive::Deserialize;
use toml;
use alloc::string::{String, ToString};
use std::fs;
use std::process::exit;
use std::env;

#[derive(Deserialize)]
struct Data{
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    polpath: String,
    binpath: String,
    firstbin: String,
    secondbin: String,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = "config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Couldn't read file {}", filename);  
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,

        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    
    let pol_path = data.config.polpath + "\\usr\\all\\";
    let bin_path = data.config.binpath;
    let firstbin = data.config.firstbin;
    let secondbin = data.config.secondbin;
    let firstfile = format!("{}{}", bin_path, firstbin);
    let secondfile = format!("{}{}", bin_path, secondbin);

    let mut buffer = String::new();

    if args.len() < 2 {
        println!("Which file to load.\n1 or 2");

        std::io::stdin().read_line(&mut buffer);

        if buffer.trim() == "1"{
            println!("Switching login_w.bin to {}", firstbin);
            fs::copy(pol_path.to_string() + "login_w.bin", secondfile);
            fs::copy(firstfile, pol_path.to_string() + "login_w.bin");

        }
        else if buffer.trim() == "2"{
            println!("Switching login_w.bin to {}", secondbin);
            fs::copy(pol_path.to_string() + "login_w.bin", firstfile);
            fs::copy(secondfile, pol_path.to_string() + "login_w.bin");
        }
        else {
            println!("WRONG FILE NUMBER ENTERED")
        }
        exit(1);
    }

    
    if args[1].trim() == "1"{
        println!("Switching login_w.bin to {}", firstbin);
        fs::copy(pol_path.to_string() + "login_w.bin", secondfile);
        fs::copy(firstfile, pol_path.to_string() + "login_w.bin");
    }
    else if args[1].trim() == "2"{
        println!("Switching login_w.bin to {}", secondbin);
        fs::copy(pol_path.to_string() + "login_w.bin", firstfile);
        fs::copy(secondfile, pol_path.to_string() + "login_w.bin");
    }
    else {
        println!("Incorrect argument entered")
    }
    
}

