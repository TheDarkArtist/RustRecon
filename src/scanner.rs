
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub async fn scan_port(addr: SocketAddr, tx: mpsc::Sender<u16>) {
    match tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => {
            if let Err(_) = tx.send(addr.port()).await {
                eprintln!("Error sending port result");
            }
        }
        _ => {}
    }
}

