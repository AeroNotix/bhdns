use crate::dns::protocol::Packet;
use std::convert::TryFrom;
use std::net::UdpSocket;

fn handle_query(socket: &UdpSocket) -> Result<(), std::io::Error> {
    let mut buf = [0; 65535];
    let (_, src) = socket.recv_from(&mut buf)?;

    let packet = Packet::try_from(Vec::from(buf)).unwrap();
    for question in packet.questions {
        println!("{:?}", question);
    }

    // To make things work, just use quad 8 and copy
    socket.send_to(&buf[0..512], "8.8.8.8:53")?;
    socket.recv_from(&mut buf)?;
    socket.send_to(&buf[0..512], src)?;

    Ok(())
}

pub fn serve() -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;
    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
