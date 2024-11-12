use std::path::{Path, PathBuf};

pub struct DirCrawler {
  dir_path: PathBuf,
  is_exist: bool,
  is_dir: bool
}

pub struct DirCrawlerValidate {
  is_ok: bool,
  message: String
}

impl DirCrawler {
  pub fn new(_dir_path: String) -> Self {
    let path = Path::new(&_dir_path);
    Self {
      dir_path: PathBuf::from(path),
      is_exist: path.exists().to_owned(),
      is_dir: path.is_dir().to_owned()
    }
  }

  pub fn validate(&self) -> DirCrawlerValidate {
    if !&self.is_exist {
      return DirCrawlerValidate {
        is_ok: false,
        message: String::from("Path does not exist")
      }
    } else if !&self.is_dir {
      return DirCrawlerValidate {
        is_ok: false,
        message: String::from("Path is not directory")
      }
    } else {
      return DirCrawlerValidate {
        is_ok: true,
        message: String::new()
      }
    }
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