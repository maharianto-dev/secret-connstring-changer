use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::error_handler::app_error::AppError;

pub struct FileWriter {
    file_path: PathBuf,
    file_content: HashMap<String, Value>,
}

pub struct JsonConfig {
    file_exist: bool,
    content: HashMap<String, Value>,
    env: String,
}

impl FileWriter {
    pub fn new(_file_path: &str) -> Result<Self, Box<dyn Error>> {
        match FileWriter::read_json_from_content(_file_path) {
            Ok(result) => Ok(Self {
                file_path: Path::new(&_file_path).to_owned(),
                file_content: result,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn file_content(&self) -> &HashMap<String, Value> {
        &self.file_content
    }

    fn read_json_from_content(_file_path: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let content_str = fs::read_to_string(_file_path)?;
        let content = serde_json::from_str(content_str.trim_start_matches("\u{feff}"))?;
        Ok(content)
    }
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
                    json_content =
                        serde_json::from_str(content_str.as_str())?;
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
