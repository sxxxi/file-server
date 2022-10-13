use std::{path::{ Path, PathBuf }, fmt::{Display, Formatter}};

#[derive(Debug)]
enum Error {
    NotFound,
    // IsDir,
    // IsFile,
    Invalid
}

pub struct AppSpace<'a> {
    root: &'a Path,
    current: &'a Path
}

impl<'a> AppSpace<'a> {
    pub fn new(root: &'a str) -> Self {
        AppSpace { 
            root: Path::new(root),
            current: Path::new("")
        }
    }

    pub fn pwd(&self) -> PathBuf {
        self.root.join(self.current)
    }

    // List files | change directory | delete |
     

    pub fn parse_current(&mut self, path: &str) -> Result<PathBuf, Error> {
        let mut path = Path::new(path);
        let mut path_buf = PathBuf::new();
        
        path_buf.push(self.root);

        
        if path.is_absolute() {
            if path.starts_with(self.root) {
                path = path.strip_prefix(self.root).unwrap();
            }
            path = path.strip_prefix("/").unwrap();
            path_buf.push(path);
            
        } else {
            let (op, str) = AppSpace::get_initial_operator(path).unwrap();
            if let Some(op) = op {
                match op {
                    ".." => {
                        if let Some(_) = self.current.parent() {
                            path_buf.push(self.current);
                            path_buf.pop();
                        }
                    }
                    _ => {}
                }
            } else {
                path_buf.push(self.current);
            }

            path_buf.push(str);
            path = str;
        }
        
        if AppSpace::check_existence(path_buf) {
            return Ok(PathBuf::from(path));
        }

        Err(Error::NotFound)
    }

    fn get_initial_operator(path: &Path) -> Result<(Option<&str>, &Path), Error> {
        if let Some(component) = path.iter().nth(0) {
            match component.to_str() {
                // I know and I'm sorry! I'm just looking for an excuse to use match guards!
                Some(op) if op == "~" || op == ".." => {
                    return Ok((Some(op), path.strip_prefix(op).unwrap()))
                }
                Some(_) => {
                    return Ok((None, path))
                }
                None => return Err(Error::Invalid)
            }
        }
        Err(Error::Invalid)
    }

    fn check_existence(path_buf: PathBuf) -> bool {
        let path = path_buf.as_path();
        // Change to try_exists when I start working with permissions
        path.exists()
    }
}

impl Display for AppSpace<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.root.display(), self.current.display())
    }
}
