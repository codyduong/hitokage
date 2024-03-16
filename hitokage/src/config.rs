// use serde::{Deserialize, Serialize};
// use std::fs::{self, File};
// use std::io::{self, Read, Write};
// use std::path::PathBuf;

// #[derive(Debug, Serialize, Deserialize)]
// struct Config {
//     widgets: Vec<String>,
//     // Define your schema here
// }

// const DEFAULT_CONFIG: Config = Config {
//     widgets: vec![],
//     // Provide default values for your schema
// };

// fn get_config_path() -> io::Result<PathBuf> {
//     let home_dir = dirs::home_dir()
//         .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
//     let config_path = home_dir.join(".foobar").join("config.json");

//     Ok(config_path)
// }

// fn read_config() -> io::Result<Config> {
//     let config_path = get_config_path()?;

//     if !config_path.exists() {
//         println!("No configuration found, supplying with default configuration");
//         // Create the directory and file if they don't exist
//         fs::create_dir_all(config_path.parent().unwrap())?;

//         save_config(&DEFAULT_CONFIG)?;
//     }

//     let mut file = File::open(&config_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let config: Config = match serde_json::from_str(&contents) {
//         Ok(value) => value,
//         Err(e) => {
//             eprintln!(
//                 "Error reading config file: {}. Falling back to default config.",
//                 e
//             );
//             return Ok(DEFAULT_CONFIG);
//         }
//     };

//     return Ok(config);
// }

// fn save_config(config: &Config) -> io::Result<()> {
//     let config_path = get_config_path()?;
//     let mut file = File::create(&config_path)?;
//     let json = serde_json::to_string_pretty(config)?;

//     file.write_all(json.as_bytes())?;

//     Ok(())
// }

// pub(crate) fn read_config_main() {
//     match read_config() {
//         Ok(config) => println!("Config: {:?}", config),
//         Err(e) => eprintln!("Error reading config: {}", e),
//     }
// }
