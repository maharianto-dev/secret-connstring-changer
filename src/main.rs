mod args_helper;
mod content_helper;
mod dir_helper;
mod error_handler;

use args_helper::args::ProgramArgs;
use content_helper::{config_creator::{self, ConfigCreator}, file_writer::JsonConfig};
use dir_helper::dir_crawler::DirCrawler;

fn main() {
    let args = ProgramArgs::new();
    let config_creator = ConfigCreator::new("Config".to_string(),"json".to_string());
    match config_creator {
        Ok(mut con_creator) => {
            if con_creator.is_success() {
                println!("{}",con_creator.get_message());
            } else {
                con_creator.generate_new_file().unwrap();
                println!("{}",con_creator.get_message());
            };
        },
        Err(error) => {
            eprintln!("{}", error);
            eprintln!("panic!");
        }
    }
    match JsonConfig::new(args.connstring_key().to_string()) {
        Ok(json) => match json.get_config_connection_string() {
            Ok(conn_string) => {
                let dir_crawler = DirCrawler::new(&String::from(args.secret_dir()));
                match dir_crawler.validate().is_ok() {
                    true => {
                        let _ = dir_crawler.run_crawler(&conn_string);
                        println!("Done changing connection string");
                        println!("Graceful shutdown");
                    }
                    false => {
                        eprintln!("{}", dir_crawler.validate().message());
                        eprintln!("panic!");
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading config file!");
                eprintln!("panic!");
            }
        },
        Err(error) => eprintln!("JSON Config is not valid!"),
    }
}
