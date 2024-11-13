use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};

use serde_json::Value;

pub struct FileWriter {
    file_path: PathBuf,
    file_content: HashMap<String, Value>,
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

    fn read_json_from_content(_file_path: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let content_str = fs::read_to_string(_file_path)?;
        let content = serde_json::from_str(content_str.trim_start_matches("\u{feff}"))?;
        Ok(content)
    }
}
