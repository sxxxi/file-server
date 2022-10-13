use tokio::net::TcpStream;
use crate::payload::Request;

#[tokio::main]
async fn main() {
    let connection = TcpStream::connect("192.168.0.5:8080");
    let pload = serde_json::to_string::<Request>(&Request::FGet { name: "Hello".to_string()});
    connection.write_all(pload);
    connection.flush();
    
}