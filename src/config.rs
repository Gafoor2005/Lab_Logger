
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub enum LabType {
    Single,
    Double,
}

impl fmt::Display for LabType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LabType::Single => write!(f, "Single"),
            LabType::Double => write!(f, "Double"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub  struct Config {
    room_no: String,
    lab_type: LabType,
    lab_names: Vec<String>,
    lab_incharge: String,
}

pub fn get_config() -> Result<Config> {
    let file_path = "config.json";
    let config:Config;
    if Path::new(file_path).exists() {
        println!("\x1b[34mConfig File exists, reading...\x1b[0m");
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        config = serde_json::from_str(&contents)?;
        // println!("Config: {:?}", config);
        // println!("{}",config.lab_type);
    } else {
        println!("\x1b[33m=========== Config File not found, creating... =============\x1b[0m");
        config = Config {
            room_no: prompt_user("\x1b[33mEnter room no: \x1b[0m")?,
            lab_type: match prompt_user("\x1b[33mEnter lab type (single/double): \x1b[0m")? {
                s if s.to_lowercase() == "single" => LabType::Single,
                s if s.to_lowercase() == "double" => LabType::Double,
                _ =>  panic!("Invalid lab type"),
            },
            lab_names: prompt_user_vec("\x1b[33mEnter lab names (comma separated if two): \x1b[0m")?,
            lab_incharge: prompt_user("\x1b[33mEnter lab incharge name: \x1b[0m")?,
        };
        let json = serde_json::to_string_pretty(&config)?;
        let mut file = File::create(file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\x1b[32mConfig File created successfully!\x1b[0m");
    }

    Ok(config)
}

fn prompt_user(prompt: &str) -> Result<String> {
    println!("\n{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().to_string())
}

fn prompt_user_vec(prompt: &str) -> Result<Vec<String>> {
    println!("\n{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().split(',').map(|s| s.trim().to_string()).collect())
}