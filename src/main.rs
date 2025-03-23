use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Plugin {
    name: String,
    version: String,
    author: Option<String>,
    description: Option<String>,
    link: Option<String>,
    url: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    plugins: Vec<Plugin>,
}

fn load_config() -> Config {
    let mut file = File::open("nofwl.yml").expect("Failed to open nofwl.yml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read nofwl.yml");
    serde_yaml::from_str(&contents).expect("Failed to parse nofwl.yml")
}

fn initialize_plugins(config: &Config) {
    for plugin in &config.plugins {
        println!("Initializing plugin: {}", plugin.name);
        if let Some(urls) = &plugin.url {
            for url in urls {
                println!("Connecting to Val.town API endpoint: {}", url);
                // Add code to connect to Val.town API endpoints
            }
        }
    }
}

fn main() {
    let config = load_config();
    initialize_plugins(&config);
    println!("Hello, NoFWL!");
}
