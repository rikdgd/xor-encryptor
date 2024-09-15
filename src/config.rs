use std::io::{
    ErrorKind,
    Result as IoResult,
    Error as IoError
};
use std::fs;

pub struct Config {
    pub paths: Vec<String>,
    pub encryption_key: String,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> IoResult<Self> {
        match args.len() {
            3 => {
                let path = &args[1];
                let encryption_key = args[2].clone();

                // If the path leads to a directory
                if fs::read_dir(path).is_ok() { 
                    let paths = get_files_in_dir(path)?;
                    return Ok(Self {
                        paths,
                        encryption_key,
                    });
                    
                // If the path leads to a file    
                } else if fs::read(path).is_ok() {
                    let paths= vec![path.to_string()];
                    return Ok(Self {
                        paths,
                        encryption_key,
                    });
                }

                Err(IoError::new(
                    ErrorKind::InvalidInput,
                    "Invalid amount of arguments provided."
                ))
            },
            _ => {
                Err(IoError::new(
                    ErrorKind::InvalidInput, 
                    "Invalid amount of arguments provided."
                ))
            }
        }
    }
}

fn get_files_in_dir(dir_path: &str) -> IoResult<Vec<String>> {
    let read = fs::read_dir(dir_path)?;
    let mut files = Vec::new();
    for dir_entry in read.flatten() {
        let path = dir_entry
            .path()
            .to_str()
            .unwrap()
            .to_string();
        
        // Only add files inside the directory, not subdirectories.
        if fs::read_dir(&path).is_err() {
            files.push(path);
        }
    }
    
    Ok(files)
}
