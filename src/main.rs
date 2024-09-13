mod xor_encryptor;

use std::io::stdin;
use xor_encryptor::XorEncryptor;

fn main() -> std::io::Result<()>{
    println!("This is a simple XOR encryption binary.\nDO NOT USE THIS TO HIDE ANY \
    SENSITIVE DATA AS THIS METHOD OF ENCRYPTION IS VERY VULNERABLE!");
    
    println!("Please enter the path to the file that should be encrypted: ");
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
    
    Ok(())
}
