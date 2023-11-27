use std::fs;
use std::fs::DirEntry;

pub fn read_dir(path: &str) -> std::io::Result<Vec<DirEntry>> {
    let dir = fs::read_dir(path)?;
    let mut contents = Vec::new();
    for e in dir {
        contents.push(e.unwrap())
    }

    Ok(contents)
}
