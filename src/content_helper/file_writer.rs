use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};

use serde::Deserialize;
use serde_json::Value;

use crate::error_handler::app_error::AppError;

pub struct JsonConfig {
    file_exist: bool,
    content: HashMap<String, Value>,
    env: String,
}

impl JsonConfig {
    pub fn new(strenv: String) -> Result<Self, Box<dyn Error>> {
        let current_dir = JsonConfig::get_current_working_dir()?;
        let contents = fs::read_dir(current_dir)?;
        let mut config_exist = false;
        let mut json_content: HashMap<String, Value> = HashMap::new();

        for content in contents {
            let dir_content = &content?;
            if dir_content.file_name() == "Config.json" {
                config_exist = true;
                if config_exist {
                    let content_str = fs::read_to_string(dir_content.path())?;
                    json_content = serde_json::from_str(content_str.as_str())?;
                }
            }
        }

        Ok(Self {
            file_exist: config_exist,
            content: json_content,
            env: strenv,
        })
    }

    pub fn get_config_connection_string(&self) -> Result<String, Box<dyn Error>> {
        match self.content.get(&self.env) {
            Some(result) => {
                return Ok(result.to_string());
            }
            None => {
                return Err(Box::new(AppError("Env Key not found!".into())));
            }
        }
    }

    fn get_current_working_dir() -> std::io::Result<PathBuf> {
        env::current_dir()
    }
}

pub struct FileWriterRaw {
    file_path: PathBuf,
    file_content: String,
}

impl FileWriterRaw {
    pub fn new(_file_path: &str) -> Result<Self, Box<dyn Error>> {
        match fs::read_to_string(_file_path) {
            Ok(result) => Ok(Self {
                file_path: Path::new(&_file_path).to_owned(),
                file_content: result,
            }),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn replace_connstring(&self, new_conn_string: &String) -> Result<(), Box<dyn Error>> {
        let mut new_content = String::new();
        let content_lines = self.file_content.split("\n");
        for line in content_lines {
            let mut new_line = line.to_string();
            if line.contains("Data Source=") {
                let conn_string_key_value: Vec<&str> = line.split(":").collect();
                new_line = format!("{}: {}", conn_string_key_value[0], new_conn_string);
                if line.trim().ends_with(",") {
                    new_line = format!("{},", new_line);
                }
            }
            new_content = format!("{}{}\n", new_content, new_line);
        }
        fs::write(&self.file_path, new_content)?;
        Ok(())
    }
}
