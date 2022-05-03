use crate::dns::protocol::Packet;
use std::convert::TryFrom;
use tokio::net::UdpSocket;

async fn handle_query(socket: &UdpSocket) -> Result<(), std::io::Error> {
    let mut buf = [0; 512];

    let (_, src) = socket.recv_from(&mut buf).await?;

    let packet = Packet::try_from(Vec::from(buf)).unwrap();
    dbg!(packet.questions);

    // To make things work, just use quad 8 and copy
    socket.send_to(&buf, "8.8.8.8:53").await?;
    socket.recv_from(&mut buf).await?;
    socket.send_to(&buf, src).await?;

    Ok(())
}

pub async fn serve() -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053)).await?;
    loop {
        match handle_query(&socket).await {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
