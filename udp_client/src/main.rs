use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Create the socket we send from
    let client_socket = UdpSocket::bind("127.0.0.1:2048").expect("couldn't bind to address");

    let message = "Hello world!";

    client_socket.send_to(message.as_bytes(), "127.0.0.1:12000").expect("couldn't send data");

    let mut buf = [0; 12];

    let (amt, src) = client_socket.recv_from(&mut buf)?;

    let buf = &mut buf[..amt];
    buf.reverse();

    let response = match std::str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", response);
    Ok(())
}
