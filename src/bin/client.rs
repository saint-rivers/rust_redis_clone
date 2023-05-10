use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    // stream.write_all(b"set foo bar").await?;
    stream.write_all(b"get foo").await?;

    let mut buf = BytesMut::with_capacity(1024);
    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            // if resp == "r Ok" {
            //     println!("key updated");
            // } else if resp == "Ok" {
            //     println!("key set");
            // }

            if resp == "" {
                println!("no such key found");
            } else {
                println!("value: {}", resp);
            }
        }
        Err(err) => {
            // failed to convert bytes into string slice
            println!("error: {}", err);
        }
    }

    Ok(())
}
