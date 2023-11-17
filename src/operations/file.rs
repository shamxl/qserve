use std::fs;
use std::io::Read;

pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    Ok(contents)
}
