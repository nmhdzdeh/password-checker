use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

pub fn save_password_to_file<P: AsRef<Path>>(path: P, password: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "{}", password)?;
    Ok(())
}
