use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::io::Result as StdResult;
use std::io::Error as StdError;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct XorEncryptor {
    file_path: String,
    key: String,
    key_length: usize,
}
impl XorEncryptor {
    pub fn new(file_path: String, key: String) -> Self {
        let key_length = key.len();
        
        Self { 
            file_path, 
            key,
            key_length
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
        println!("File has been successfully encrypted!");
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
        file.set_len(0)?; // TODO: causes overflow. Replace with OpenOptions EVERYWHERE
        file.write_all(&to_write)?;
        file.flush()?;
        
        Ok(())
    }
    
    fn xor_chunk(&self, chunk: Vec<u8>) -> StdResult<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();
        if chunk.len() != self.key_length {
            // TODO: Trim the key to the same size instead of returning an error.
            return Err(StdError::new(
                ErrorKind::InvalidData, 
                "key didn't match chunk length"
            ));
        } else {
            for i in 0..chunk.len() {
                let data_byte = chunk.get(i).unwrap();
                let key_byte = chunk.get(i).unwrap();
                
                result.push(data_byte ^ key_byte);
            }
        }
        
        Ok(result)
    }
    
}
