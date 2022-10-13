use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use crate::payload::{ Request, Response, Error };

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read_request(&mut self) -> Result<Request, Error> {
        let mut serialized = String::new();
        self.stream.read_to_string(&mut serialized).await.unwrap();

        match serde_json::from_str::<Request>(serialized.as_str()) {
            Ok(req) => Ok(req),
            _ => {
                eprintln!("Unimplemented");
                Err(Error::Other)
            }
        }
    }

    pub async fn write_response(&mut self, response: Response) -> Result<(), Error> {
        let buf = serde_json::to_string::<Response>(&response).unwrap();
        match self.stream.write_all(buf.as_bytes()).await {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::Other)
        }
    }
}