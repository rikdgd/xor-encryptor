mod xor_encryptor;
mod config;

use std::io::stdin;
use std::env;
use xor_encryptor::XorEncryptor;
use config::Config;


const SUCCESS_MESSAGE: &str = "\nSuccessfully encrypted/decrypted files!";

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(config) = Config::from_args(args) {
        let mut encryptor = XorEncryptor::from_config(config);
        encryptor.encrypt().expect("Failed to encrypt file(s).");
        println!("{SUCCESS_MESSAGE}");
        
    } else {
        guided_setup();
    }
}


fn guided_setup() {
    println!("This is a simple XOR encryption/decryption binary.\n\nDO NOT USE THIS TO HIDE ANY \
    SENSITIVE DATA AS THIS METHOD OF ENCRYPTION IS EXTREMELY VULNERABLE!\n");

    println!("Please enter the path to the file that should be encrypted/decrypted: ");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).expect("Failed to read user input.");

    let file_path = file_path.trim().to_string();
    if std::fs::read(&file_path).is_err() && std::fs::read_dir(&file_path).is_err() {
        panic!("The given path was invalid.");
    }

    println!("Now enter a key to use for encryption: ");
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Failed to read user input.");
    let key = key.trim().to_string();
    
    let config = Config::from_path(&file_path, &key).expect("The given path is invalid.");
    let mut encryptor = XorEncryptor::from_config(config);
    
    encryptor.encrypt().expect("An error occurred during the encryption process.");
    
    println!("{SUCCESS_MESSAGE}");
}
