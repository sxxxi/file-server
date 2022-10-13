use std::path::{ Path, PathBuf, Component }; 
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
        let path = Path::new(path);           // The passed argument as Path to perform necessary operations.
        let mut no_root = PathBuf::from(self.current        
            .strip_prefix(self.root.clone())
            .unwrap());                              // PWD stripped off of the root path

        // Check every component for operations.       
        let mut component_pos: usize = 0;
        for c in path.components() {
            match c {
                Component::RootDir => {
                    //This happens 0 or 1 times
                    path_buf.push(self.root.clone());
                } 
                Component::ParentDir => {
                    no_root.pop();
                }
                Component::Normal(s) if s == "~" => {   // No support for tilde based on the docs :(
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
                _ => {} // I don't think i'm gonna support Windows 
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
