use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;


#[derive(Deserialize, Debug)]
pub struct Data {
    pub config: Config,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub src: String,
    pub main: String
}


pub fn get_toml() -> Data {
    let filename = "fiskur.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("could not find/read fiskur.toml");
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("could not load data from fiskur.toml!");
            exit(1);
        }
    };

    return data;
}
