use std::fs::File;
use std::io::Error;

pub fn write_file(file_path: &str) -> Result<File, Error> {
    let file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", file_path, e);
            return Err(e);
        }
    };

    Ok(file)
}
