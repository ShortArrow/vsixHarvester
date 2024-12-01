use std::fs;
use std::path::Path;

pub fn create(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
