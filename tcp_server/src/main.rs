use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    // Create the socket we listen on
    let listener = TcpListener::bind("127.0.0.1:2048").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response: String = request.join("").to_uppercase();
    println!("Response: {}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
