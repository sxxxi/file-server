use std::{path::{ Path, PathBuf, Component }, ffi::OsStr};
use std::fmt::{ Formatter, Display };


#[derive(Debug)]
pub enum Error {
    NotFound,
    // IsDir,
    // IsFile,
    InvalidOp
}

pub struct AppSpace {
    root: PathBuf,
    pub current: PathBuf
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
        let mut path_buf = PathBuf::new();
        let path = Path::new(path);

        
        // To be used for file path operations. ('..', '~', '.')
        let mut no_root = PathBuf::from(self.current.strip_prefix(self.root.clone()).unwrap());

        // Check every component for operations. Return Error::InvalidOp if something goes wrong.
        let mut component_pos: usize = 0;
        for c in path.components() {
            match c {
                Component::RootDir => {
                    //This happens 0 or 1 time
                    path_buf.push(self.root.clone());
                } 
                Component::ParentDir => {
                    no_root.pop();
                }
                // No support for tilde based on the docs :(
                Component::Normal(s) if s == "~" => { 
                    if component_pos == 0 {
                        path_buf.push(self.root.clone());
                        no_root.clear();
                    } else {
                        return Err(Error::InvalidOp)
                    }
                } 
                Component::Normal(s) => {
                    no_root.push(s);
                }
                // I don't think i'm gonna support Windows 
                _ => {}
            }
            component_pos += 1;
        }
        path_buf.push(no_root);
        Ok(path_buf)

    }

    // File Ops: Create/delete file/dir. List directory contents, mv, cp
    // pub fn crate_file() 
    
}

impl Display for AppSpace {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.root.display(), self.current.display())
    }
}
