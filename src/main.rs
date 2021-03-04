use async_std::{io, net::TcpStream, prelude::*};

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6379").await?;
    let command = b"*1\r\n$4\r\nPING\r\n";
    stream.write_all(command).await?;
    let mut buffer = vec![0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    // println!("{:?}", std::str::from_utf8(&buffer[..bytes_read]));
    println!("{:?}", parse_response(&buffer[0..bytes_read]).unwrap());
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, String> {
    if buffer.is_empty() {
        return Err("Empty buffer".into());
    }

    if buffer[0] == ('-' as u8 ) {
        return Err(format!("Error Response: {:?}", &buffer[1..buffer.len() - 2]));
    }

    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}
