mod server;
mod payload;
mod connection;
mod fops;

use std::{fs, io};
use std::io::{prelude::*, Error};

#[tokio::main]
async fn main() {
    let mut buf = String::new();
    let mut fs = fops::AppSpace::new("/home/domino/"); 
    
    
    loop {
        println!("What do?");
        println!("1. Create file");
        println!("2. Create directory");
        println!("3. Delete fd");
        println!("4. List");
        get_input(&mut buf).unwrap();
        println!("{buf}");

        match buf.as_str().trim() {
            "list" => {
                println!("matched");
                for e in fs.list().unwrap() {
                    println!("{:?}", e.unwrap().file_name());
                }
            }
            _ => {}
        }

        // if buf == "1".to_string() {
        //     println!("Enter file name:");
        //     get_input(&mut buf).unwrap();
        //     fs.create_file(buf.as_str()).unwrap();
        // }

        // if buf == "2".to_string() {
        //     println!("Enter directory name:");
        //     get_input(&mut buf).unwrap();
        //     fs.create_dir(buf.as_str(), true).unwrap();
        // }

        // if buf == "3".to_string() {
        //     println!("Enter fd name:");
        //     get_input(&mut buf).unwrap();
        //     fs.delete_fd(buf.as_str()).unwrap();
        // }

        // if buf == "4".to_string() {
        // }

        println!("Press Enter");
        get_input(&mut String::new()).unwrap();
        print!("\x1B[2J\x1B[1;1H");
        
    }
}

fn get_input<'a>(buf: &'a mut String) -> Result<(), Error> {
    buf.clear();
    match io::stdin().read_line(buf) {
        Err(error) => Err(error),
        Ok(_) => Ok(()) 
    }
}