use std::{
    env, 
    error::Error,
    fs::{self, File}, path::{Path, PathBuf},
};

pub struct ConfigCreator {
    file_name: String,
    file_extension: String,
    dir: PathBuf,
    is_success: bool,
    message: String,
}

impl ConfigCreator {
    pub fn new(_file_name: String, _file_ext: String) -> Result<Self, Box<dyn Error>> {
        let current_exe_path = env::current_exe()?;
        let curr_dir = current_exe_path.parent().unwrap();

        let mut is_success: bool = false;
        let mut message: String = "File not found!".to_owned();
        let mut directory = curr_dir.to_path_buf();
        let contents = fs::read_dir(curr_dir)?;
        for content in contents {
            let dir_content = &content?;
            if *dir_content.file_name() == *format!("{}.{}",_file_name,_file_ext) {
                is_success = true;
                message = "File exist!".to_owned();
            }
        }
        Ok(Self {
            file_name: _file_name,
            file_extension: _file_ext,
            dir: directory,
            is_success: is_success,
            message: message,
        })
    }
    pub fn is_success(&self) -> bool {
        return self.is_success;
    }
    pub fn get_message(&self) -> String {
        return self.message.clone();
    }
    pub fn generate_new_file(&mut self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.{}",self.file_name, self.file_extension);
        println!("Generating File {}...", file_name);
        let file_path = self.dir.join(file_name);
        match File::create(file_path) {
            Ok(_) => {
                self.is_success = true;
                self.message = format!("{}.{} success to create!", self.file_name, self.file_extension);
                
                Ok(())
            }
            Err(error) => {
                return Err(Box::new(error));
            }
        }
    }
}
