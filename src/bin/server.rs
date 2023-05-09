use tokio::{net::TcpListener, io::AsyncReadExt};

use bytes::BytesMut;

#[tokio::main]
// async fn main() {
//     println!("in main")
// }

// fn main() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async { println!("in main") })
// }

pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connection accepted {:?}", socket);

        let mut buf = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buf).await?;
        println!("buffer {:?}", buf);
    }
    // Ok(())
}
