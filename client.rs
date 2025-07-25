use std::env;
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: client <nom_de_domaine>");
        std::process::exit(1);
    }

    let domaine = &args[1];
    let socket = UdpSocket::bind("0.0.0.0:0")?; 
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    socket.send_to(domaine.as_bytes(), "127.0.0.1:8080")?;

    let mut buf = [0u8; 512];
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => {
            let response = String::from_utf8_lossy(&buf[..size]);
            println!("Réponse: {}", response);
        }
        Err(e) => eprintln!("Erreur: pas de réponse - {}", e),
    }

    Ok(())
}
