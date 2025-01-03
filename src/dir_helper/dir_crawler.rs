use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::content_helper::file_writer::FileWriterRaw;

pub struct DirCrawler {
    dir_path: PathBuf,
    is_exist: bool,
    is_dir: bool,
}

pub struct DirCrawlerValidate {
    is_ok: bool,
    message: String,
}

impl DirCrawler {
    pub fn new(_dir_path: &str) -> Self {
        let path = Path::new(&_dir_path);
        Self {
            dir_path: PathBuf::from(path),
            is_exist: path.exists().to_owned(),
            is_dir: path.is_dir().to_owned(),
        }
    }

    pub fn validate(&self) -> DirCrawlerValidate {
        if !&self.is_exist {
            return DirCrawlerValidate {
                is_ok: false,
                message: String::from("Path does not exist"),
            };
        } else if !&self.is_dir {
            return DirCrawlerValidate {
                is_ok: false,
                message: String::from("Path is not directory"),
            };
        } else {
            return DirCrawlerValidate {
                is_ok: true,
                message: String::new(),
            };
        }
    }

    pub fn run_crawler(&self, conn_string: &String) -> Result<(), Box<dyn Error>> {
        let main_dir = &self.dir_path;
        let contents = fs::read_dir(main_dir)?;
        for content in contents {
            let dir_content = &content?;
            let file_path = dir_content.path().to_str().unwrap().to_owned();
            if dir_content.file_name() == "secrets.json" {
                // potential panic here when reading file content, let it be
                match FileWriterRaw::new(&file_path) {
                    Ok(filewriter) => {
                        filewriter.replace_connstring(conn_string)?;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                };
            }
            let child_dir = DirCrawler::new(&file_path);
            if child_dir.is_dir {
                let _ = child_dir.run_crawler(conn_string);
            }
        }
        Ok(())
    }
}

impl DirCrawlerValidate {
    pub fn is_ok(&self) -> &bool {
        &self.is_ok
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
