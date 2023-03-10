use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    /*for i in 0..100 {
        let mut stream = TcpStream::connect("127.0.0.1:8089").await.unwrap();
        stream.write_all(format!("Hello, server! from iter {}",i).as_bytes()).await.unwrap();
    }*/
    Ok(())
}
