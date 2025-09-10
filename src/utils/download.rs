use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use reqwest::blocking::Client;


/// Downloads a file from a given URL and saves it to the given output file.
///
/// `url`: The URL of the file to download.
///
/// `output_file`: The file path where the downloaded file will be saved.
///
/// `callback`: A callback function that will be called once per block
///             of data received from the server. The callback will be
///             given two arguments: the number of bytes that have been
///             downloaded so far, and the total size of the file.
///
pub fn new<F>(
    url: &str, 
    output_file: &str, 
    callback: F
) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn(usize, usize),

{
    let client = Client::new();

    let response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Request failed: {}", e).into()),
        
    };

    let total_size = match response.content_length() {
        Some(len) => len,
        None => return Err("Can't determine content length".into()),
    };

    let path = Path::new(output_file);
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Failed to create file: {}", e).into()),
    };

    let mut source = BufReader::new(response);
    let mut buffer = [0u8; 8192];
    let mut downloaded = 0;

    loop {
        callback(downloaded as usize, total_size as usize);
        let bytes_read = match source.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(format!("Read error: {}", e).into()),
        };

        if let Err(e) = file.write_all(&buffer[..bytes_read]) {
            return Err(format!("Write error: {}", e).into())
        }

        downloaded += bytes_read as u64;
    }

    Ok(())
}