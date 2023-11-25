use std::fs;
use std::fs::DirEntry;
use std::io::Read;

pub fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    Ok(contents)
}

pub fn read_dir(path: &str) -> std::io::Result<Vec<DirEntry>> {
    let dir = fs::read_dir(path)?;
    let mut contents = Vec::new();
    for e in dir {
        contents.push(e.unwrap())
    }

    Ok(contents)
}
