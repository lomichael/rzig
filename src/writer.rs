use std::fs::File;
use std::io::{self, Write};

pub fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())
}
