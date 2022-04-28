use std::convert::{TryFrom};
use std::net::UdpSocket;
use crate::dns::protocol::Header;

fn handle_query(socket: &UdpSocket) -> Result<(), std::io::Error> {
    let mut buf = [0; 512];

    let (_, src) = socket.recv_from(&mut buf)?;

    println!("{:?}", buf);

    let header = Header::try_from(&buf[0 .. 12]).unwrap();
    println!("{}", header);
    println!("{:?}", header.response_code);


    socket.send_to(&buf, "8.8.8.8:53")?;
    socket.recv_from(&mut buf)?;
    socket.send_to(&buf, src)?;

    Ok(())
}

pub fn serve() -> Result<(), std::io::Error> {
    // Bind an UDP socket on port 2053
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    // For now, queries are handled sequentially, so an infinite loop for servicing
    // requests is initiated.
    loop {
        match handle_query(&socket) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
