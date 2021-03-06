use super::config_model::Config;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::*;

pub(super) fn load_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let path = match args.len() {
        2 => {
            debug!("Path to config: {}", args.get(1).unwrap());
            args.get(1).expect("Get config path")
        }

        x => {
            println!(
                "Wrong number of arguments:{}\nDefault config will be applied",
                x
            );
            ""
        }
    };

    let path = Path::new(path);

    if path.exists() && path.is_file() {
        let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {:?}", path));
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .unwrap_or_else(|_| panic!("Failed to read file content: {:?}", path));
        match serde_json::from_str(&file_content) {
            Ok(config) => config,
            Err(err) => {
                error!("{}", err);
                Config::default()
            }
        }
    } else {
        debug!("Path either doesn't exist or is not a file");
        Config::default()
    }
}
