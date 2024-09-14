mod xor_encryptor;
mod config;

use std::io::stdin;
use std::env;
use xor_encryptor::XorEncryptor;
use config::Config;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Ok(config) = Config::from_args(args) {
        let mut encryptor = XorEncryptor::from_config(config);
        encryptor.encrypt()?;
        println!("Successfully encrypted file!");
        
        Ok(())
    } else {
        guided_setup()
    }
}


fn guided_setup() -> std::io::Result<()> {
    println!("This is a simple XOR encryption/decryption binary.\n\nDO NOT USE THIS TO HIDE ANY \
    SENSITIVE DATA AS THIS METHOD OF ENCRYPTION IS VERY VULNERABLE!\n");

    println!("Please enter the path to the file that should be encrypted/decrypted: ");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path)?;

    let file_path = file_path.trim().to_string();
    if let Err(_) = std::fs::File::open(&file_path) {
        panic!("Invalid file path.");
    }

    println!("Now enter a key to use for encryption: ");
    let mut key = String::new();
    stdin().read_line(&mut key)?;

    let key = key.trim().to_string();
    if key.len() < 6 {
        panic!("Given key was to short.");
    }

    let mut encryptor = XorEncryptor::new(file_path, key);
    encryptor.encrypt()?;
    println!("File has been successfully encrypted/decrypted!");

    Ok(())
}
