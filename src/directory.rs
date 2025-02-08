use std::error::Error;
use std::fs;
use std::path::Path;

pub trait FileSystem {
    fn create_dir_all(&mut self, directory: &Path) -> Result<(), Box<dyn Error>>;
    fn path_exists(&self, directory: &Path) -> bool;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&mut self, directory: &Path) -> Result<(), Box<dyn Error>> {
        if !directory.exists() {
            fs::create_dir_all(directory)?;
        }
        Ok(())
    }
    fn path_exists(&self, directory: &Path) -> bool {
        directory.exists()
    }
}

pub fn create_dir_all(directory: &str) -> Result<(), Box<dyn Error>> {
    let mut fs = RealFileSystem;
    create_dir_all_with_fs(&mut fs, directory)
}

pub fn create_dir_all_with_fs(fs: &mut dyn FileSystem, directory: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(directory);
    if !fs.path_exists(path) {
        fs.create_dir_all(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::path::Path;

    struct MockFileSystem {
        path_exists: bool,
        create_dir_all_called: bool,
    }
    impl FileSystem for MockFileSystem {
        fn create_dir_all(&mut self, _path: &Path) -> Result<(), Box<dyn Error>> {
            self.create_dir_all_called = true;
            Ok(())
        }
        fn path_exists(&self, _path: &Path) -> bool {
            self.path_exists
        }
    }
    #[test]
    fn test_create_dir_all() {
        let mut mock_fs = MockFileSystem {
            path_exists: false,
            create_dir_all_called: false,
        };
        create_dir_all_with_fs(&mut mock_fs, "test").unwrap();
        assert!(mock_fs.create_dir_all_called);
    }
    #[test]
    fn test_not_create_dir_all() {
        let mut mock_fs = MockFileSystem {
            path_exists: true,
            create_dir_all_called: false,
        };
        create_dir_all_with_fs(&mut mock_fs, "test").unwrap();
        assert!(!mock_fs.create_dir_all_called);
    }
}
