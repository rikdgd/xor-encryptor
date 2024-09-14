use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::io::Result as StdResult;
use crate::Config;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct XorEncryptor {
    file_path: String,
    key: Vec<u8>,
    key_length: usize,
}
impl XorEncryptor {
    pub fn new(file_path: String, key: String) -> Self {
        let key_length = key.len();
        let key = key.as_bytes().to_vec();
        
        Self { 
            file_path,
            key,
            key_length
        }
    }
    
    pub fn from_config(config: Config) -> Self {
        let key_length = config.encryption_key.len();
        let key = config.encryption_key.as_bytes().to_vec();
        
        Self {
            file_path: config.file_path,
            key,
            key_length,
        }
    }

    pub fn encrypt(&mut self) -> StdResult<()> {
        let file_bytes = self.read_file_bytes()?;
        let mut encrypted_bytes: Vec<u8> = Vec::new();
        
        for chunk in file_bytes.chunks(self.key_length) {
            let chunk = chunk.to_vec();
            let xor_chunk = self.xor_chunk(chunk)?;
            for byte in xor_chunk {
                encrypted_bytes.push(byte);
            }
        }
        
        self.clear_write_file(encrypted_bytes)?;
        Ok(())
    }
    
    fn read_file_bytes(&self) -> StdResult<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.file_path)?;
        let bytes_read = file.read_to_end(&mut buffer)?;
        file.flush()?;
        println!("Bytes to encrypt: {}", bytes_read);
        
        Ok(buffer)
    }
    
    fn clear_write_file(&self, to_write: Vec<u8>) -> StdResult<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        file.set_len(0)?;
        file.write_all(&to_write)?;
        file.flush()?;
        
        Ok(())
    }
    
    fn xor_chunk(&self, chunk: Vec<u8>) -> StdResult<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();

        for i in 0..chunk.len() {
            let data_byte = chunk.get(i).unwrap();
            let key_byte = self.get_key_byte(&i);

            result.push(data_byte ^ key_byte);
        }
        
        Ok(result)
    }
    
    fn get_key_byte(&self, index: &usize) -> &u8 {
        let mut index = *index;
        while index > self.key_length {
            index -= self.key_length;
        }
        
        self.key.get(index).expect("Failed to get key byte.")
    }
}
