use tokio::{net::TcpStream, io::AsyncWriteExt};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    stream.write_all(b"set foo bar").await?;

    Ok(())
}
