use async_std::{net::TcpStream, io};

#[async_std::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:6379").await?;

    println!("hellooo");
    Ok(())
}
