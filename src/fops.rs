use std::path::{ Path, PathBuf, Component }; 
use std::fmt::{ Formatter, Display };
use std::fs::{ self, DirBuilder, File };

use tokio::io::copy;

#[derive(Debug)]
pub enum Error {
    NotFound,
    NotDirectory,
    NotFile,
    InvalidOp,
    InvalidPath,
    FileExists,
    DirectoryExists,
    FileOperationFailed,
    Other
}

pub struct AppSpace {
    root: PathBuf,
    current: PathBuf
}

impl AppSpace {
    pub fn new(root: &str) -> Self {
        AppSpace { 
            root: PathBuf::from(root),
            current: PathBuf::from(root)
        }
    }

    pub fn pwd(&self) -> PathBuf {
        self.current.clone()
    }

    // Just parse. No checks. Let the file ops do it later.
    pub fn parse_path(&mut self, path: &str) -> Result<PathBuf, Error> {
        let mut path_buf = PathBuf::new();  // The base buffer. Returned at the end.
        let path = Path::new(path);         // The passed argument as Path to perform necessary operations.
        let mut no_root = PathBuf::from(self.current        
            .strip_prefix(self.root.clone())
            .unwrap());                              // PWD stripped off of the root path

        // Check every component for operations.       
        let mut component_pos: usize = 0;
        for c in path.components() {
            match c {
                //This happens 0 or 1 times
                Component::RootDir => { path_buf.push(self.root.clone()); }                
                Component::ParentDir => { no_root.pop(); }
                Component::Normal(s) if s == "~" => {   // No support for tilde based on the docs :(
                    if component_pos == 0 {
                        path_buf.push(self.root.clone());
                        no_root.clear();
                    } else {
                        return Err(Error::InvalidOp)
                    }
                } 
                Component::Normal(s) => { no_root.push(s); }
                _ => {} // I don't think i'm gonna support Windows 
            }
            component_pos += 1;
        }
        path_buf.push(no_root);
        Ok(path_buf)

    }

    pub fn list(&self) -> Result<fs::ReadDir, Error> {
        if let Ok(rd) = fs::read_dir(self.current.clone()) {
            return Ok(rd)
        } else {
            // This io::Result will be an Err if there’s some sort of intermittent IO error during iteration.
            return Err(Error::Other)
        }
    }

    // cd
    pub fn change_dir(&mut self, path: &str) -> Result<(), Error> {
        let valid_path = self.parse_path(path).unwrap();

        if !valid_path.exists() {
            return Err(Error::NotFound)
        }

        if !valid_path.is_dir() {
            return Err(Error::NotDirectory)
        }
    
        Ok(self.current = valid_path)
    }

    // cp
    pub fn copy_fd(&mut self, from: &str, to: &str) -> Result<(), Error> {
        let from = self.parse_path(from).unwrap();
        let mut to = self.parse_path(to).unwrap();

        // You cant just copy/move root >:(
        if !from.exists() || from == self.root  {
            return Err(Error::NotFound)
        }

        if to.exists() {
            if to.is_dir() {
                to.push(from.file_name().unwrap());
            }

            //you cant move things to files/links
            return Err(Error::FileExists)
        }

        fs::copy(from, to).unwrap();
        Ok(())
    }

    pub fn create_file(&mut self, path:&str) -> Result<(), Error> {
        self.parse_path(path).and_then(|path| {
            if path.exists() {
                return Err(Error::FileExists)
            }

            match File::create(path) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::FileOperationFailed)
            }
            
        }).or_else(|e| Err(e))
    }

    pub fn create_dir(&mut self, path: &str, recursive: bool) -> Result<(), Error> {
        self.parse_path(path).and_then(|path| {
            let mut db = DirBuilder::new();
            db.recursive(recursive);
            if path.exists() {
                //I'd just throw an error regardless
                return Err(Error::DirectoryExists)
            }
            fs::create_dir_all(path).or_else(|_| Err(Error::FileOperationFailed))
        }).or_else(|e| Err(e))
    } 

    pub fn delete_fd(&mut self, file: &str) -> Result<(), Error> {
        self.parse_path(file).and_then(|file| {
            if !file.exists() {
                return Err(Error::NotFound)
            }

            // This function will return an error in the following situations, but is not limited to just these cases:
            // path points to a directory. OK
            // The file doesn’t exist. OK
            // The user lacks permissions to remove the file. <To be implemented in the future>
            if file.is_dir() {
                fs::remove_dir_all(file).or_else(|_| Err(Error::FileOperationFailed))
            } else {
                fs::remove_file(file).or_else(|_| Err(Error::FileOperationFailed)) 
            }
        }).or_else(|e| Err(e))
    }
    
    // mv - should work with directories as well
    pub fn move_file(&mut self, from: &str, to: &str) -> Result<(), Error> {
        if let Err(e) = self.copy_fd(from, to) {
            return Err(e);
        };
        if let Err(e) = self.delete_fd(to) {
            return Err(e);
        };
        
        Ok(())
    }
}

impl Display for AppSpace {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.root.display(), self.current.display())
    }
}
