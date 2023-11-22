use std::fs;
use std::io::Read;

pub fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    Ok(contents)
}
