use std::net::UdpSocket;

fn main() -> std::io::Result<()> { 
    // Create the socket we listen on
    let server_socket = UdpSocket::bind("127.0.0.1:12000").expect("couldn't bind to address");

    loop {
        let mut buf = [0; 12];
        let (amt, _src) = server_socket.recv_from(&mut buf)?;
        
        
        let buf = &mut buf[..amt];
        buf.reverse();
        
        let response = match std::str::from_utf8(buf) {
            Ok(v) => v.to_uppercase(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        server_socket.send_to(response.as_bytes(), "127.0.0.1:2048").expect("couldn't send data");
    }
    Ok(())
}
