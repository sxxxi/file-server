use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use serde::{ Serialize, Deserialize };

pub enum Error {
    Other
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    FList,
    FGet {
        name: String,
    },
    FPut,
}

impl Request {
    pub async fn read_request(stream: &mut TcpStream) -> Result<Request, Error>{
        let mut serialized = String::new();
        stream.read_to_string(&mut serialized).await.unwrap();

        match serde_json::from_str::<Request>(serialized.as_str()) {
            Ok(req) => Ok(req),
            _ => {
                eprintln!("Unimplemented");
                Err(Error::Other)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    
}