use std::io::{
    ErrorKind,
    Result as IoResult,
    Error as IoError
};

pub struct Config {
    pub file_path: String,
    pub encryption_key: String,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> IoResult<Self> {
        match args.len() {
            3 => {
                Ok(Self {
                    file_path: args[1].clone(),
                    encryption_key: args[2].clone(),
                })
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
