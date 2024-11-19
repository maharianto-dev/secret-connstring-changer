use std::{
    env, 
    error::Error,
    fs::File, 
    path::{Path, PathBuf},
};

pub struct ConfigCreator {
    file_name: String,
    file_extension: String,
    message: String,
}

impl ConfigCreator {
    pub fn new(_file_name: String, _file_ext: String) -> Result<Self, Box<dyn Error>> {
        let curr_dir = env::current_dir()?;
        let full_filename = format!("{}.{}",&_file_name,&_file_ext);
        let config_file = PathBuf::from(curr_dir.join(Path::new(&full_filename)));
        match config_file.exists() && config_file.is_file() {
            true => Ok(Self {
                file_name: _file_name,
                file_extension: _file_ext,
                message: "File exist!".to_string(),
            }),
            false => {
                ConfigCreator::generate_new_file(&config_file)?;
                return Ok(Self {
                    file_name: _file_name,
                    file_extension: _file_ext,
                    message: format!("{} success to generate!", &full_filename),
                });
            }
        
        }
        
    }
    
    fn generate_new_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
        println!("Generating File {:?}...", path.file_name().expect("File not found!"));
        File::create(path)?;
        Ok(())
    }
}
