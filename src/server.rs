use tokio::{net::TcpListener, time::error::Error}; 
use crate::payload::Request;

// Server will execute commands based on Command type
pub async fn serve(socket_addr: &str) -> Result<(), Error> {
    // Listen for incoming requests
    let listener = TcpListener::bind(socket_addr).await.unwrap(); 

    // Accept requests
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        use crate::payload::Request::*;

        match Request::read_request(&mut stream).await {
            Ok(FList) => {

            }
            Ok(FGet { name }) => {
                println!("Received: {:?}", name);
            }
            Ok(FPut) => {

            }
            _ => eprintln!("TODO")
        }

        // Read request
    }
}