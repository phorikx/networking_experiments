use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Create the stream we send from
    let mut stream = TcpStream::connect("127.0.0.1:2048")?;
    let message = "Hello world!\n\n";

    stream.write_all(message.as_bytes()).unwrap();

    let mut buf = [0; 12];

    stream.read(&mut buf)?;

    let response = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", response);
    Ok(())
}
